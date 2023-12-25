use crate::common::base_structure::{Vid, Edge, Data};
use crate::parallel::server::SyncCommunicationer;
use crate::parallel::communication::communication_server::Communication;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait Pratition {
    fn vertex_partition(&self, vid : &Vid) -> usize;
    fn edge_partition<EDATA>(&self, edge : &Edge<EDATA>) -> usize;
}

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

trait Graph {
    type EDATA;
    fn new(edges : Vec<Edge<Self::EDATA>>);
}

struct NearEdge<EDATA> {
    to : Vid,
    data : EDATA,
}

struct NearGraph<EDATA> {
    g : Vec<Vec<NearEdge<EDATA>>>
}

impl<EDATA> NearEdge<EDATA> {
    fn new(edges : Vec<Edge<String>>) {
        todo!()
    }
}