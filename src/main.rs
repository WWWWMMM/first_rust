use bincode::{Encode, Decode};
use common::base_structure::Edge;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator, IndexedParallelIterator, ParallelExtend};

use crate::io::{FileRead, ReadOption, example::MyEDATA};

use std::{fmt::Debug, sync::Arc};

pub mod util;
pub mod traits;
pub mod common;
pub mod io; 
pub mod parallel;
pub mod graph;
pub mod algo;

fn main() {

}
