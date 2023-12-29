use std::net::SocketAddr;
use std::sync::Arc;
use std::marker::Send;
use std::time::Duration;
use std::time::Instant;

use bincode::Decode;
use bincode::Encode;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::task::JoinSet;
use tokio_stream::StreamExt;
use tonic::Streaming;
use tonic::transport::Endpoint;
use tonic::transport::Server;
use tonic::{Request, Response};

use super::communication;
use communication::communication_server::{Communication, CommunicationServer};
use communication::communication_client::CommunicationClient;
use communication::{Bytes, Ack};

use crate::common::constance::MAX_BYTE_SEND_PER_MSG;
use crate::graph::ClusterInfo;
use crate::util::Serilazer;

struct Receiver {
    channel : Arc<Mutex<Sender<Bytes>>>,
}

#[tonic::async_trait]
impl Communication for Receiver {
    async fn send(
        &self,
        request: tonic::Request<Streaming<communication::Bytes>>,
    ) -> std::result::Result<tonic::Response<Ack>, tonic::Status> {
        let mut stream = request.into_inner();
        let mut count = 0;
        let mut from = u32::MAX;
        println!("recv begin.");
        while let Some(bytes) = stream.next().await {
            let bytes = bytes?;
            count += bytes.val.len();
            if from != u32::MAX {
                assert_eq!(from, bytes.from);
            }
            from = bytes.from;
            let channel = self.channel.lock().await;
            channel.send(bytes).await.unwrap();
        }   
        Ok(Response::new(Ack{msg : format!("recv rank {from}: {count} bytes.")}))
    }
}

pub trait MyMpi {
    /// 返回进程个数
    fn partitions(&self) -> usize;

    fn get_cluster_info(&self) -> &ClusterInfo;

    // 将msgs[i] 发送到rank i, 同时接受所有收到的消息, 返回值中第i个值为rank i发来的消息
    fn send_recv_binary(&self, msgs : Vec<Vec<u8>>) -> Vec<Vec<u8>>;

    // 将msgs[i] 发送到rank i, 同时接受所有收到的消息, 返回值中第i个值为rank i发来的消息
    fn send_recv<MSG>(&self, msgs : Vec<MSG>) -> Vec<MSG>
    where 
        MSG : Encode + Decode + Send,
        Vec<MSG>: IntoParallelIterator<Item = MSG>,
    ;

    fn reduce<DATA>(&self, data : DATA, f : impl Fn(DATA, DATA) -> DATA) -> DATA
    where
        DATA : Encode + Decode + Send + Clone
    ;
}

#[derive(Debug)]
pub struct SyncCommunicationer {
    addr : SocketAddr,
    cluster_info : ClusterInfo,
    endpoints : Vec<Endpoint>, 
}

impl MyMpi for SyncCommunicationer {
    fn send_recv_binary(&self, msgs : Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        println!("\na send_recv");

        let t0 = Instant::now();
        let mut msgs = msgs;
        let mut all_length = msgs.iter().map(|x| x.len()).fold(0u64, |a, b| a + b as u64);

        // let rt  = Runtime::new().unwrap();
        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_io()
            .enable_time()
            .build()
            .unwrap();
        rt.block_on(async {
            let partitions = self.endpoints.len();
            
            let mut res = vec![vec![]; partitions];

            let (send, mut recv) = mpsc::channel::<Bytes>(partitions);
            let (shotdown_send, shotdown_recv) = oneshot::channel::<()>();

            // start server
            let addr = self.addr;
            let svc = CommunicationServer::new(Receiver {
                channel : Arc::new(Mutex::new(send)),
            });

            let mut set = JoinSet::new();
            set.spawn(async move {
                Server::builder().add_service(svc).serve_with_shutdown(addr, async {
                    let _ = shotdown_recv.await;
                    ()
                }).await.unwrap();
            });
            
            tokio::time::sleep(Duration::from_micros(5)).await;
            // send msg
            for i in 0..msgs.len() {
                if i == self.cluster_info.rank {
                    // println!("send msg to self length: {:?}", msgs[i].len());
                    res[i] = std::mem::take(&mut msgs[i]);
                }else {
                    let endpoint = self.endpoints[i].clone();
                    let msg = std::mem::take(&mut msgs[i]);
                    let rank = self.cluster_info.rank as u32;
                    println!("rank {} spawn send task.", self.cluster_info.rank);
                    set.spawn(async move {
                        let tmp = Instant::now();
                        let mut client = loop {
                            let a = CommunicationClient::connect(endpoint.clone()).await;
                            if a.is_ok() {
                                break a.unwrap()
                            }else {
                                println!("waiting connect.");
                            }
                        };
                        println!("connect cost: {:?}", Instant::now() - tmp);

                        let tmp2 = Instant::now();
                        let mut splited_msg = msg.chunks(MAX_BYTE_SEND_PER_MSG).map(|x| {
                            Bytes {val : x.to_vec(), from : rank, finish : false}
                        }).collect::<Vec<Bytes>>();
                        splited_msg.last_mut().unwrap().finish = true;
                        match client.send(Request::new(tokio_stream::iter(splited_msg))).await {
                            Ok(response) => println!("{} recv Ack: {:?}", rank, response.into_inner()),
                            Err(e) => println!("something went wrong: {:?}", e),
                        };

                        println!("send msg to one p cost: {:?}", Instant::now() - tmp2);
                    });
                }
            }

            // recv msg
            let tmp = Instant::now();
            let mut finish = vec![false; partitions];
            finish[self.cluster_info.rank] = true;
            let mut finish_count = 1;
            while let Some(mut bytes) = recv.recv().await {
                assert!(finish[bytes.from as usize] == false);
                res[bytes.from as usize].append(&mut bytes.val);
                if bytes.finish {
                    finish[bytes.from as usize] = true;
                    finish_count += 1;
                    if finish_count == partitions {
                        break;
                    }
                }
            }
            println!("recv all msgs cost: {:?}", Instant::now() - tmp);

            // shutdown server
            println!("shotdown: {}", self.cluster_info.rank);
            shotdown_send.send(()).unwrap();
            while let Some(_) = set.join_next().await {}

            println!("send {all_length} cost: {:?}\n", Instant::now() - t0);
            res
        })
    }

    fn send_recv<MSG>(&self, msgs : Vec<MSG>) -> Vec<MSG>
    where 
        MSG : Encode + Decode + Send,
        Vec<MSG>: IntoParallelIterator<Item = MSG>,
     {
        let t0 = Instant::now();

        let serilazer = Serilazer::new();
        let msgs : Vec<Vec<u8>> = msgs.into_par_iter().map(|x|{
            serilazer.encode(x)
        }).collect();
        // println!("encode cost: {:?}", Instant::now() - t0);

        let recv = self.send_recv_binary(msgs);

        let t1 = Instant::now();
        let res = recv.into_par_iter().map(|x|{
            serilazer.decode(x)
        }).collect();
        // println!("decode cost: {:?}", Instant::now() - t1);

        res
    }

    fn reduce<DATA>(&self, data : DATA, f : impl Fn(DATA, DATA) -> DATA) -> DATA 
    where
        DATA : Encode + Decode + Send + Clone
    {
        let partitions = self.endpoints.len();
        let msgs = vec![data; partitions];

        let recv = self.send_recv(msgs);

        recv.into_iter().reduce(f).unwrap()
    }

    fn get_cluster_info(&self) -> &ClusterInfo {
        &self.cluster_info
    }

    fn partitions(&self) -> usize {
        self.endpoints.len()
    }
}

pub fn com_for_test(port1 : i32, port2 : i32, rank : usize) -> SyncCommunicationer{
    SyncCommunicationer {
        addr: format!("[::1]:1000{}", if rank == 0 {port1} else {port2}).parse().unwrap(),
        cluster_info: ClusterInfo { partitions : 2, rank : rank},
        endpoints: vec![Endpoint::from_shared(format!("http://[::1]:1000{port1}").to_string()).unwrap(), 
        Endpoint::from_shared(format!("http://[::1]:1000{port2}").to_string()).unwrap()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LEN : usize = 1000000;

    #[derive(Debug, Encode, Decode, Clone, PartialEq)]
    struct TestMsg {
        a : i32,
        b : f32,
        c : String,
    }

    #[test]
    fn send_recv0() {
        let communicatoner =  com_for_test(0, 1, 0);
        let msg = vec![vec![TestMsg{a : 0, b : 0.0, c : "0.0.0".into()}; LEN]; 2];
        let recv = communicatoner.send_recv(msg);

        assert_eq!(recv, vec![vec![TestMsg{a : 0, b : 0.0, c : "0.0.0".into()}; LEN],
                        vec![TestMsg{a : 1, b : 1.0, c : "1.0.0".into()}; LEN]]);
    }

    #[test]
    fn send_recv1() {
        let communicatoner =  com_for_test(0, 1, 1);
        let msg = vec![vec![TestMsg{a : 1, b : 1.0, c : "1.0.0".into()}; LEN]; 2];
        let recv = communicatoner.send_recv(msg);

        assert_eq!(recv, vec![vec![TestMsg{a : 0, b : 0.0, c : "0.0.0".into()}; LEN],
                        vec![TestMsg{a : 1, b : 1.0, c : "1.0.0".into()}; LEN]]);
    }
}