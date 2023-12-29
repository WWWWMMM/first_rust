use bincode::{Encode, Decode};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, ParallelDrainFull, IntoParallelIterator, IndexedParallelIterator, ParallelExtend};

use crate::common::base_structure::{Vid, Edge, Eid};
use crate::parallel::server::MyMpi;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::fmt::Debug;

#[derive(Debug)]
pub struct GraphInfo {
    pub vertex_num : Vid,
    pub edge_num : Eid,
}

#[derive(Debug)]
pub struct ClusterInfo {
    pub partitions : usize,
    pub rank : usize,
}

fn process_vec<T: Send + Sync>(data: &Vec<T>) {
    data.par_iter().for_each(|item| {
        // 在这里处理每个元素 `item`
        // 例如打印元素值
        // println!("{:?}", item);
    });
}

impl GraphInfo {
    fn from<EDATA>(edges : &Vec<Edge<EDATA>>, communication : &impl MyMpi) -> Self
    where 
        EDATA : Sync
    {
        let max_id : Vid = edges.par_iter().map(|x| x.from.max(x.to)).max().unwrap_or_default();
        let local_edge : Eid = edges.len() as Eid;

        let vertex_num = 1 + communication.reduce(max_id, |a, b| {
            a.max(b)
        });
        let edge_num = communication.reduce(local_edge, |a, b|{
            a + b
        });

        GraphInfo {
            vertex_num : vertex_num,
            edge_num : edge_num
        }
    }
}

pub trait Pratition : Debug {
    fn vertex_partition(&self, vid : &Vid) -> usize;
    fn edge_partition<EDATA>(&self, edge : &Edge<EDATA>) -> usize;
}

pub trait SeqPartition : Pratition {
    fn new(degrees : Vec<Vid>, graph_info : &GraphInfo, cluster_info : &ClusterInfo) -> Self;
    fn impl_partition<EDATA>(&self, edges : Vec<Edge<EDATA>>, communication : &impl MyMpi) -> Vec<Edge<EDATA>>
    where
        EDATA : Clone + Send + Debug,
        Vec<Edge<EDATA>> : IntoParallelIterator<Item = Edge<EDATA>> + Encode + Decode;

    fn start_id(&self) -> Vid;
    fn end_id(&self) -> Vid;
}

#[derive(Debug)]
struct HashPartition {}

impl HashPartition {
    fn hash(&self, vid : &Vid) -> u64{
        let mut s = DefaultHasher::new();
        vid.hash(&mut s);
        s.finish()
    }
}

impl Pratition for HashPartition {
    fn vertex_partition(&self, vid : &Vid) -> usize {
        self.hash(vid) as usize
    }

    fn edge_partition<EDATA>(&self, edge : &Edge<EDATA>) -> usize {
        self.hash(&edge.from) as usize
    }
}

#[derive(Debug)]
pub struct SeqSPartition {
    rank : usize,
    end_id : Vec<Vid>
}

impl Pratition for SeqSPartition {
    fn vertex_partition(&self, vid : &Vid) -> usize {
        self.end_id.partition_point(|&end_id|  end_id <= *vid)
    }

    fn edge_partition<EDATA>(&self, edge : &Edge<EDATA>) -> usize {
        self.vertex_partition(&edge.from)
    }
}

impl SeqPartition for SeqSPartition {
    fn new(degrees : Vec<Vid>, graph_info : &GraphInfo, cluster_info : &ClusterInfo) -> Self {
        let v = graph_info.vertex_num;
        let mut end_id = vec![];
        let chunk = v / cluster_info.partitions as u32 + 1;
        for i in 1..cluster_info.partitions as u32 {
            end_id.push(chunk * i);
        }
        end_id.push(v);
        SeqSPartition { rank : cluster_info.rank, end_id: end_id }
    }

    fn impl_partition<EDATA>(&self, edges : Vec<Edge<EDATA>>, communication : &impl MyMpi) -> Vec<Edge<EDATA>> 
    where
        EDATA : Clone + Send + Debug,
        Vec<Edge<EDATA>> : IntoParallelIterator<Item = Edge<EDATA>> + Encode + Decode,
    {
        println!("impl partition");
        let partitions = communication.get_cluster_info().partitions;
        let msgs = 
            edges
            .into_par_iter()
            .fold(
                ||{
                    vec![vec![]; partitions]
                }, 
                |mut a, b|{
                    let p1 = self.vertex_partition(&b.from);
                    let p2 = self.vertex_partition(&b.to);
                    if p1 != p2 {
                        a[p1].push(b.clone());
                        a[p2].push(b);
                        
                    }else {
                        a[p2].push(b);
                    }
                    a
                })
            .reduce(
                ||{
                    vec![vec![]; partitions]
                }, 
                |mut vec1, vec2| {
                    vec1
                        .into_par_iter()
                        .zip(vec2.into_par_iter())
                        .map(
                            |(mut v1, v2)| {
                                v1.par_extend(v2);
                                v1
                            })
                        .collect()
                }
            );
        // println!("{:?}", msgs);
        let recv = communication.send_recv::<Vec<Edge<EDATA>>>(msgs);
        
        recv.into_par_iter().flatten().collect()
    }

    fn start_id(&self) -> Vid {
        if self.rank == 0 {
            0
        }else {
            self.end_id[self.rank - 1]
        }
    }   

    fn end_id(&self) -> Vid {
        self.end_id[self.rank]
    }
}

pub trait Graph {
    type PART : SeqPartition;
    fn local_vertexs(&self) -> Vid;

    // 获取一个数组。
    fn get_array<T>(&self, init_data : T) -> Vec<T>
    where
        T : Clone;
   
    /// 每个顶点的度数
    fn degrees(&self) -> Vec<Vid>;

    /// 返回这玩意
    fn partition(&self) -> &Self::PART;
}

#[derive(Clone, Debug)]
pub struct NearEdge<EDATA> {
    pub to : Vid,
    data : EDATA,
}

#[derive(Debug)]
pub struct NearGraph<EDATA, PART> 
where
    PART : SeqPartition + Sync,
    EDATA : Clone + Send + Sync,
    Vec<Edge<EDATA>> : IntoParallelIterator<Item = Edge<EDATA>> + Encode + Decode,
{
    pub graph_info : GraphInfo,
    pub g : Vec<Vec<NearEdge<EDATA>>>,
    partition : PART,
}

impl<EDATA, PART> NearGraph<EDATA, PART> 
where
    PART : SeqPartition + Sync,
    EDATA : Clone + Send + Sync + Debug,
    Vec<Edge<EDATA>> : IntoParallelIterator<Item = Edge<EDATA>> + Encode + Decode,
{
    pub fn nbr(&self, id : usize) -> &Vec<NearEdge<EDATA>> {
        &self.g[id]
    }

    pub fn new(edges : Vec<Edge<EDATA>>, communication : &impl MyMpi) -> Self 
    {
        let graph_info = GraphInfo::from(&edges, communication);
        let cluster_info = communication.get_cluster_info();
        let partition = PART::new(vec![], &graph_info, cluster_info);
        let edges = partition.impl_partition(edges, communication);
        // println!("{:?}", edges);
        let global_vertexs = graph_info.vertex_num as usize;
        println!("builg g");
        
        let mut g: Vec<Vec<_>> = vec![vec![]; global_vertexs];
        edges.into_iter().for_each(|b| {
            g[b.from as usize].push(NearEdge{to : b.to, data : b.data.clone() });
            g[b.to as usize].push(NearEdge{to : b.from, data : b.data });
        });

        // 这个并行方式并不行
        // let g = edges
        //     .into_par_iter()
        //     .fold(
        //         ||{
        //             vec![vec![]; global_vertexs]
        //         }, 
        //         |mut a, b|{
        //             a[b.from as usize].push(NearEdge{to : b.to, data : b.data.clone() });
        //             a[b.to as usize].push(NearEdge{to : b.from, data : b.data });
        //             a
        //         })
        //     .reduce(
        //         ||{
        //             vec![vec![]; global_vertexs]
        //         }, 
        //         |mut vec1, vec2| {
        //             vec1
        //                 .into_par_iter()
        //                 .zip(vec2.into_par_iter())
        //                 .map(
        //                     |(mut v1, v2)| {
        //                         v1.par_extend(v2);
        //                         v1
        //                     })
        //                 .collect()
        //         }
        //     );
        let build_result = NearGraph {
            graph_info : graph_info,
            g : g,
            partition : partition
        };
        println!("builg g over");
        // println!("{:?}", build_result);
        build_result
    }
}

impl<EDATA, PART> Graph for NearGraph<EDATA, PART> 
where
    PART : SeqPartition + Sync,
    EDATA : Clone + Send + Sync,
    Vec<Edge<EDATA>> : IntoParallelIterator<Item = Edge<EDATA>> + Encode + Decode,
{
    type PART = PART;
    fn degrees(&self) -> Vec<Vid> {
        (self.partition.start_id()..self.partition.end_id()).map(|i|{
            self.g[i as usize].len() as Vid
        }).collect()
    }

    fn get_array<T>(&self, init_data : T) -> Vec<T> 
    where
        T : Clone
    {
        vec![init_data; self.local_vertexs() as usize]
    }

    fn local_vertexs(&self) -> Vid {
        self.g.len() as Vid
    }

    fn partition(&self) -> &Self::PART 
    where
        PART : SeqPartition 
    {
        &self.partition
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parallel::server::*, io::{*, csv::*, example::*}};
    const LEN : usize = 1000000;

    #[derive(Debug, Encode, Decode, Clone, PartialEq)]
    struct TestMsg {
        a : i32,
        b : f32,
        c : String,
    }

    #[test]
    fn send_recv0() {
        let communicatoner = com_for_test(2, 3, 0);
        
        // read from rank 0
        let a = CsvReader::new();
        let mut read = ReadOption::default();
        read.include_columns = vec!["i32_data".into(), "f32_data".into(), "str_data".into()];
        read.header = "from:uint,to:uint,i32_data:int,f32_data:f32,str_data:string".into();
        let edges = a.read_edge::<MyEDATA>("data/graph_example.csv".into(), read);

        let graph = NearGraph::<MyEDATA, SeqSPartition>::new(edges, &communicatoner);
        println!("{:?}", graph);
    }

    #[test]
    fn send_recv1() {
        let communicatoner = com_for_test(2, 3, 1);
        let edges = vec![];

        let graph = NearGraph::<MyEDATA, SeqSPartition>::new(edges, &communicatoner);
        println!("{:?}", graph);
    }
}