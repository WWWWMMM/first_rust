use super::data::Data;
use super::vertex::Vid;

#[derive(Debug)]
pub struct Edgeuus {
    pub from : Vid,
    pub to : Vid,
    pub data : String,
}

#[derive(Debug)]
pub struct Edge {
    pub from : Vid,
    pub to : Vid,
    pub data : Data
}