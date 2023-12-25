use super::vertex::Vid;

#[derive(Debug)]
pub struct Edge<T> {
    pub from : Vid,
    pub to : Vid,
    pub data : T,
}