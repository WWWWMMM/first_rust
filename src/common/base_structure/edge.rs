use arrow::array::ArrayRef;
use arrow_schema::{Schema, Field, DataType};
use bincode::{Encode, Decode};

use super::vertex::Vid;

pub type Eid = u64;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct Edge<T> {
    pub from : Vid,
    pub to : Vid,
    pub data : T,
}