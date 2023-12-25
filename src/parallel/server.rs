use std::net::SocketAddr;
use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::task::JoinSet;
use tonic::transport::Endpoint;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use routeguide::communication_server::{Communication, CommunicationServer};
use routeguide::communication_client::CommunicationClient;
use routeguide::{Bytes, Ack};

pub mod routeguide {
    tonic::include_proto!("communication");
}
use crate::common::constance::MAX_BYTE_SEND_PER_MSG;

struct Receiver {
    channel : Arc<Mutex<Sender<Bytes>>>,
}

#[tonic::async_trait]
impl Communication for Receiver {
    async fn send(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Ack>, Status> {
        let bytes = request.into_inner();
        let recv_len = bytes.val.len().to_string();
        {
            let channel = self.channel.lock().await;
            channel.send(bytes).await.unwrap();
        }
        Ok(Response::new(Ack{msg : recv_len}))
    }
}

trait MyMpi {
    // 将msgs[i] 发送到rank i, 同时接受所有收到的消息, 返回值中第i个值为rank i发来的消息
    fn send_recv(&self, msgs : Vec<Vec<u8>>) -> Vec<Vec<u8>>;
}

#[derive(Debug)]
pub struct SyncCommunicationer {
    addr : SocketAddr,
    rank : usize,
    endpoints : Vec<Endpoint>, 
}

impl MyMpi for SyncCommunicationer {
    
    fn send_recv(&self, msgs : Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut msgs = msgs;
        let rt  = Runtime::new().unwrap();
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
            
            // send msg
            for i in 0..msgs.len() {
                if i == self.rank {
                    println!("send msg to self length: {:?}", msgs[i].len());
                    res[i] = std::mem::take(&mut msgs[i]);
                }else {
                    let endpoint = self.endpoints[i].clone();
                    let msg = std::mem::take(&mut msgs[i]);
                    let rank = self.rank as u32;
                    println!("rank {} spawn send task.", self.rank);
                    set.spawn(async move {
                        let mut client = CommunicationClient::connect(endpoint).await.unwrap();
                        let mut start = 0;
                        let mut end;
                        loop {
                            end = std::cmp::min(start + MAX_BYTE_SEND_PER_MSG, msg.len());
                            let bytes = Bytes {
                                val : msg[start..end].to_vec(),
                                from : rank,
                                finish : end == msg.len()
                            };
                            match client.send(Request::new(bytes)).await {
                                Ok(response) => println!("{} recv Ack: {:?}", rank, response.into_inner()),
                                Err(e) => println!("something went wrong: {:?}", e),
                            };
                            start += MAX_BYTE_SEND_PER_MSG;
                            if start >= msg.len() {
                                break;
                            }
                        }
                    });
                }
            }

            // recv msg
            let mut finish = vec![false; partitions];
            finish[self.rank] = true;
            let mut finish_count = 1;
            while let Some(mut bytes) = recv.recv().await {
                res[bytes.from as usize].append(&mut bytes.val);
                if bytes.finish {
                    assert!(finish[bytes.from as usize] == false);
                    finish[bytes.from as usize] = true;
                    finish_count += 1;
                    if finish_count == partitions {
                        break;
                    }
                }
            }

            // shutdown server
            println!("shotdown: {}", self.rank);
            shotdown_send.send(()).unwrap();
            while let Some(_) = set.join_next().await {}

            res
        })
    }
}

#[cfg(test)]
mod tests {
    use tonic::transport::Endpoint;

    use crate::parallel::server::MyMpi;

    use super::SyncCommunicationer;
    const LEN : usize = 5000000;
    #[test]
    fn test_send_recv0() {
        let communicatoner =  SyncCommunicationer {
            addr: "[::1]:10000".parse().unwrap(),
            rank: 0,
            endpoints: vec![Endpoint::from_static("http://[::1]:10000"), Endpoint::from_static("http://[::1]:10001")],
        };
        let msg = vec![vec![0u8; LEN]; 2];
        let mut recv = communicatoner.send_recv(msg);

        recv.sort();
        assert_eq!(recv[0].len(), LEN);
        assert_eq!(recv[1].len(), LEN);
        assert_eq!(recv, vec![vec![0u8; LEN], vec![1u8; LEN]]);
    }

    #[test]
    fn test_send_recv1() {
        let communicatoner =  SyncCommunicationer {
            addr: "[::1]:10001".parse().unwrap(),
            rank: 1,
            endpoints: vec![Endpoint::from_static("http://[::1]:10000"), Endpoint::from_static("http://[::1]:10001")],
        };
        let msg = vec![vec![1u8; LEN]; 2];
        let mut recv = communicatoner.send_recv(msg);

        recv.sort();
        assert_eq!(recv[0].len(), LEN);
        assert_eq!(recv[1].len(), LEN);
        assert_eq!(recv, vec![vec![0u8; LEN], vec![1u8; LEN]]);
    }
}