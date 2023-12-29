use bincode::{Encode, Decode};
use std::{fmt::Debug, time::Instant};
use rayon::iter::{IntoParallelRefMutIterator, IntoParallelRefIterator, ParallelIterator, IntoParallelIterator, IndexedParallelIterator};

use crate::{graph::{Graph, SeqPartition, NearGraph}, common::{base_structure::{Edge, Vid}, util::SharedPtr}, parallel::server::MyMpi};

pub fn pagerank<EDATA, PART>(graph : NearGraph<EDATA, PART>, communication : &impl MyMpi) -> Vec<f32> 
where
    PART : SeqPartition + Sync,
    EDATA : Clone + Send + Sync + Debug,
    Vec<Edge<EDATA>> : IntoParallelIterator<Item = Edge<EDATA>> + Encode + Decode,
{

    let vertexs = graph.graph_info.vertex_num as usize;
    let start_id = graph.partition().start_id() as usize;
    let end_id = graph.partition().end_id() as usize;

    let local_degree = graph.degrees();
    // println!("local_degree: {:?}", local_degree);
    let mut local_pr : Vec<f32> = vec![1.0; local_degree.len()];

    let global_degree : Vec<u32> = {
        let msgs = vec![local_degree; communication.partitions()];

        let recv = communication.send_recv::<Vec<u32>>(msgs);

        recv.into_par_iter().flatten().collect()
    };
    // println!("global_degree: {:?}", global_degree);

    let iteration = 100;
    let damping = 0.85;
    let p = SharedPtr::new(local_pr.as_mut_ptr());
    for i in 0..iteration {
        let t00 = Instant::now();
        println!("iter: {i}");
        let mut t0 = Instant::now();
        let msgs = vec![local_pr.clone(); communication.partitions()];
        println!("prepare msgs cost: {:?}", Instant::now() - t0);

        t0 = Instant::now();
        let recv = communication.send_recv::<Vec<f32>>(msgs);
        println!("send_recv cost: {:?}", Instant::now() - t0);

        t0 = Instant::now();
        let global_pr : Vec<f32> = recv.into_par_iter().flatten().collect();
        println!("get global_pr cost: {:?}", Instant::now() - t0);
        // println!("get global_pr {:?}", global_pr);

        t0 = Instant::now();
        (start_id..end_id).into_par_iter().for_each(|id|{
            let nbr = graph.nbr(id);
            let mut sum = 0.0;
            nbr.iter().for_each(|edge| {
                sum += global_pr[edge.to as usize] / global_degree[edge.to as usize] as f32;
            });
            // println!("id: {id} sum: {sum} bnr: {:?}", nbr);
            unsafe {
                *p.add(id - start_id) = 1.0 - damping + damping * sum;
            };
        });
        println!("calc local pr cost: {:?}", Instant::now() - t0);
        // println!("calc local prr {:?}", local_pr);

        println!("------------------------------iter {i} cost: {:?}", Instant::now() - t00);
    }

    local_pr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parallel::server::*, io::{*, csv::*, example::*}, graph::SeqSPartition};

    #[test]
    fn send_recv0() {
        let communicatoner: SyncCommunicationer = com_for_test(5, 6, 0);
        
        // read from rank 0
        let a = CsvReader::new();
        let mut read = ReadOption::default();
        read.header = "from:uint,to:uint".into();
        let edges = a.read_edge::<MyEmpty>("data/tmp.csv".into(), read);

        println!("read compelete");

        let graph = NearGraph::<MyEmpty, SeqSPartition>::new(edges, &communicatoner);

        println!("graph build compelete");

        let pr = pagerank(graph, &communicatoner);

        println!("rank 0: {:?}", pr);
    }

    #[test]
    fn send_recv1() {
        let communicatoner = com_for_test(5, 6, 1);
        let edges = vec![];

        let graph = NearGraph::<MyEmpty, SeqSPartition>::new(edges, &communicatoner);

        println!("graph build compelete");

        let pr = pagerank(graph, &communicatoner);

        println!("rank 1: {:?}", pr);
    }
}