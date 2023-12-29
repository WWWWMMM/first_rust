use lib::{parallel::server::com_for_test, graph::{SeqSPartition, NearGraph}, io::example::MyEmpty, algo::pagerank};
fn main() {
    let communicatoner = com_for_test(5, 6, 1);
    let edges = vec![];

    let graph = NearGraph::<MyEmpty, SeqSPartition>::new(edges, &communicatoner);

    // println!("graph build compelete");

    let pr = pagerank(graph, &communicatoner);

    // println!("rank 1: {:?}", pr);
}