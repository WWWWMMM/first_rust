use arrow::array::ArrayRef;
use arrow_schema::{Schema, Field, DataType};

use super::vertex::Vid;

#[derive(Debug, PartialEq)]
pub struct Edge<T> {
    pub from : Vid,
    pub to : Vid,
    pub data : T,
}