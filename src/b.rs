use std::time::Instant;

use lib::{parallel::server::{com_for_test, SyncCommunicationer}, graph::{SeqSPartition, NearGraph}, io::{example::MyEmpty, csv::CsvReader, ReadOption, FileRead}, algo::pagerank};

fn main() {
    let communicatoner: SyncCommunicationer = com_for_test(5, 6, 0);
    
    // read from rank 0
    let t0 = Instant::now();
    let a = CsvReader::new();
    let mut read = ReadOption::default();
    read.header = "from:uint,to:uint".into();
    let edges = a.read_edge::<MyEmpty>("data/1000w.csv".into(), read);

    println!("read cost: {:?}", Instant::now() - t0);

    let graph = NearGraph::<MyEmpty, SeqSPartition>::new(edges, &communicatoner);

    println!("graph build compelete");

    let t0 = Instant::now();
    let pr = pagerank(graph, &communicatoner);

    println!("pagerank cost: {:?}", Instant::now() - t0);
    // println!("rank 0: {:?}", pr);
}
