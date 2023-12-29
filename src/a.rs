use lib::{parallel::server::com_for_test, graph::{SeqSPartition, NearGraph}, io::example::MyEmpty, algo::pagerank};
fn main() {
    let communicatoner = com_for_test(5, 6, 1);
    let edges = vec![];

    let graph = NearGraph::<MyEmpty, SeqSPartition>::new(edges, &communicatoner);

    // println!("graph build compelete");

    let pool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    let pr = pool.install(||pagerank(graph, &communicatoner));

    for i in 0..10 {
        println!("{i}: {}", pr[i]);
    }
    // println!("rank 1: {:?}", pr);
}