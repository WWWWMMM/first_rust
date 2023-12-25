use crate::common::base_structure::*;
use flate2::read::GzDecoder;
pub mod data;
pub mod csv;
use data::*;

pub trait FileRead {
    fn read_edge(&self, path : String, option : ReadOption)->(Vec<Edge<String>>, Header);
    fn read_vertex(&self, path : String, option : ReadOption)->(Vec<Vertexus>, Header);
}

pub struct ReadOption {
    /// 数据包括的列，不包括顶点id
    include_columns : Vec<String>,
    
    /// 边表的起点或者点表的点
    from_column : String,

    /// 边表的终点
    to_column : String,
}

impl ReadOption {
    fn default() -> Self {
        ReadOption {
            include_columns : vec![],

            from_column : "from".into(),

            to_column : "to".into()
        }
    }
}