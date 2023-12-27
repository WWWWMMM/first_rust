use std::str::FromStr;

use arrow::array::ArrayRef;

use super::data::Data;

pub type Vid = u32;

pub struct Vertexus {
    id : Vid,
    data : String
}

pub struct Vertex<T> {
    id : Vid,
    data : T
}