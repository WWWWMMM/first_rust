use std::fmt::Debug;

use crate::common::base_structure::*;
use arrow::array::ArrayRef;
use flate2::read::GzDecoder;
pub mod data;
pub mod csv;
pub mod example;
use data::*;

pub trait FromArrow {
    fn from(arrow_data : Vec<ArrayRef>, len : usize) -> Vec<Self> 
        where Self: Sized; 
}

pub trait FileRead {
    fn read_edge<EDATA>(&self, path : String, option : ReadOption)-> Vec<Edge<EDATA>>
    where 
        EDATA : FromArrow;
    
    fn read_vertex<VDATA>(&self, path : String, option : ReadOption)-> Vec<Vertex<VDATA>>
    where 
        VDATA : FromArrow;
}

pub struct ReadOption {
    /// 文件第一行是否为header
    pub has_header : bool,

    /// 文件的header。如果被设置，这会覆盖文件第一行的header
    pub header : String,

    /// 数据包括的列，不包括顶点id
    pub include_columns : Vec<String>,
    
    /// 边表的起点或者点表的点
    from_column : String,

    /// 边表的终点
    to_column : String,
}

impl ReadOption {
    pub fn default() -> Self {
        ReadOption {
            has_header : true,

            header : String::default(),

            include_columns : vec![],

            from_column : "from".into(),

            to_column : "to".into()
        }
    }
}