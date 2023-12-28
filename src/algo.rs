use bincode::{Encode, Decode};
use rayon::iter::{IntoParallelRefMutIterator, IntoParallelRefIterator, ParallelIterator, IntoParallelIterator, IndexedParallelIterator};

use crate::{graph::{Graph, SeqPartition, NearGraph}, common::{base_structure::{Edge, Vid}, util::SharedPtr}, parallel::server::MyMpi};

fn pagerank<EDATA, PART>(graph : NearGraph<EDATA, PART>, communication : &impl MyMpi) -> Vec<f32> 
where
    PART : SeqPartition + Sync,
    EDATA : Clone + Send + Sync,
    Vec<Edge<EDATA>> : IntoParallelIterator<Item = Edge<EDATA>> + Encode + Decode,
{
    let vertexs = graph.graph_info.vertex_num as usize;
    let start_id = graph.partition().start_id() as usize;
    let end_id = graph.partition().end_id() as usize;

    let local_degree = graph.degrees();
    let mut local_pr : Vec<f32> = local_degree.par_iter().map(|&d| {
        if d == 0 {
            1.0
        }else {
            1.0 / d as f32
        }
    }).collect();

    let global_degree : Vec<u32> = {
        let msgs = vec![local_degree; communication.partitions()];

        let recv = communication.send_recv::<Vec<u32>>(msgs);

        recv.into_par_iter().flatten().collect()
    };

    // let iteration = 20;
    // let damping = 0.85;
    // let p = SharedPtr::new(local_pr.as_mut_ptr());
    // for i in 0..iteration {
    //     println!("\n\n\n\n");
    //     let msgs = vec![local_pr.clone(); communication.partitions()];

    //     let recv = communication.send_recv::<Vec<f32>>(msgs);

    //     let global_pr : Vec<f32> = recv.into_par_iter().flatten().collect();

    //     (start_id..end_id).into_par_iter().for_each(|id|{
    //         let nbr = graph.nbr(id);
    //         let mut sum = 0.0;
    //         nbr.iter().for_each(|edge| {
    //             sum += global_pr[edge.to as usize] / global_degree[edge.to as usize] as f32;
    //         });
    //         unsafe {
    //             *p.add(id) = 1.0 - damping + damping * sum;
    //         };
    //     }) 
    // }

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
        let edges = a.read_edge::<MyEmpty>("data/pagerank.csv".into(), read);

        println!("read compelete");

        let graph = NearGraph::<MyEmpty, SeqSPartition>::new(edges, &communicatoner);

        let pr = pagerank(graph, &communicatoner);

        println!("rank 0: {:?}", pr);
    }

    #[test]
    fn send_recv1() {
        let communicatoner = com_for_test(5, 6, 1);
        let edges = vec![];

        let graph = NearGraph::<MyEDATA, SeqSPartition>::new(edges, &communicatoner);

        let pr = pagerank(graph, &communicatoner);

        println!("rank 1: {:?}", pr);
    }
}