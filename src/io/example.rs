use arrow::array::{ArrayRef, Int32Array, StringArray, Float32Array};
use bincode::{Encode, Decode};

use super::FromArrow;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct MyEDATA {
    pub i32_data : i32,
    pub f32_data : f32,
    pub str_data : String,
}

impl FromArrow for MyEDATA {
    fn from(arrow_data : Vec<ArrayRef>, len : usize) -> Vec<Self>
        where Self: Sized 
    {
        let i32_data = arrow_data[0].as_any().downcast_ref::<Int32Array>().unwrap();
        let f32_data = arrow_data[1].as_any().downcast_ref::<Float32Array>().unwrap();
        let str_data = arrow_data[2].as_any().downcast_ref::<StringArray>().unwrap();

        (0..i32_data.len()).into_iter().map(|index|{
            MyEDATA { 
                i32_data : i32_data.value(index).into(),
                f32_data : f32_data.value(index).into(),
                str_data : str_data.value(index).into(),
            }
        }).collect()
    }
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct MyEmpty {}

impl FromArrow for MyEmpty {
    fn from(arrow_data : Vec<ArrayRef>, len : usize) -> Vec<Self>
        where Self: Sized 
    {
        vec![MyEmpty{}; len]
    }
}

#[cfg(test)]
mod tests {
    use crate::{io::{csv::CsvReader, ReadOption, FileRead}, common::base_structure::edge::Edge};

    use super::*;
    #[test]
    fn test_read() {
        // arrow_test();
        let a = CsvReader::new();
        let mut read = ReadOption::default();
        read.include_columns = vec!["i32_data".into(), "f32_data".into(), "str_data".into()];
        read.header = "from:uint,to:uint,i32_data:int,f32_data:f32,str_data:string".into();
        let edges = a.read_edge::<MyEDATA>("data/example.csv".into(), read);
        assert_eq!(edges, vec![
            Edge { from: 1, to: 2, data: MyEDATA { i32_data: 4, f32_data: 4.0, str_data: "4.00".into() } }, 
            Edge { from: 2, to: 3, data: MyEDATA { i32_data: 5, f32_data: 5.0, str_data: "5.00".into() } }]);
    }

    #[test]
    fn test_read_empty() {
        // arrow_test();
        let a = CsvReader::new();
        let mut read = ReadOption::default();
        read.header = "from:uint,to:uint".into();
        let edges = a.read_edge::<MyEmpty>("data/pagerank.csv".into(), read);
        // println!("{:?}", edges);
    }
}