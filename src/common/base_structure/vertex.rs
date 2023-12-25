use std::str::FromStr;

use super::data::Data;

#[derive(Debug, Hash)]
pub struct Vid(u32);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVidError;

impl FromStr for Vid {
    type Err = ParseVidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse::<u32>().map_err(|_| ParseVidError)?;
        Ok(Vid(val))
    }
}

pub struct Vertexus {
    id : Vid,
    data : String
}

pub struct Vertex {
    id : Vid,
    data : Data
}
