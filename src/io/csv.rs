use std::{fmt::format, sync::Arc, str::FromStr, time::Instant, fs::File, io::{BufReader, BufRead}};

use arrow::{csv::ReaderBuilder, array::{AsArray, Int32Array, Array, PrimitiveArray, ArrayRef}, datatypes::{Int32Type, UInt32Type, ArrowPrimitiveType}};
use arrow_schema::{Schema, Field, DataType};
use rayon::iter::ParallelExtend;

use super::*;

fn get_datatype(data_type : &str) ->DataType {
    match data_type {
        "" | "empty" => DataType::Null,
        "int" | "int32" | "i32" => DataType::Int32,
        "uint" | "uint32" | "u32" => DataType::UInt32,
        "long" | "int64" | "long long" | "i64" => DataType::Int64,
        "float" | "float32" | "f32" => DataType::Float32,
        "double" | "float64" | "f64" => DataType::Float64,
        "string" => DataType::Utf8,
        _ => {
            println!("data_type: {data_type}");
            unreachable!()
        }
    }
}

fn get_schema(path : &str, option : &ReadOption) -> Schema {
    let header_str : String = if option.has_header && option.header.is_empty() {
        let file = File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        reader.lines().next().unwrap().unwrap()
    }else {
        option.header.clone()
    };

    let fields : Vec<Field>= header_str.split(",").map(|str|{
        let (name, data_type) = str.split_once(":").unwrap();
        Field::new(name, get_datatype(data_type), false)
    }).collect();

    Schema::new(fields)
}

pub struct CsvReader {}

impl FileRead for CsvReader {
    fn read_edge<EDATA>(&self, path : String, option : ReadOption)-> Vec<Edge<EDATA>> 
    where
        EDATA : FromArrow
    {
        let file = std::fs::File::open(&path).unwrap();
        println!("file: {:?}", file);
        
        let schema = get_schema(&path, &option);
        println!("schema :{:?}", schema);

        let from_index = schema.index_of(&option.from_column).unwrap();
        let to_index = schema.index_of(&option.to_column).unwrap();
        let data_indexs = option.include_columns.iter().map(|x| {
            schema.index_of(&x).unwrap()
        }).collect::<Vec<usize>>();

        let projection : Vec<usize> = vec![from_index, to_index].into_iter().chain(data_indexs.into_iter()).collect();
        let mut reader =   
            ReaderBuilder::new(Arc::new(schema))
            .with_header(option.has_header)
            .with_batch_size(1024 * 1024)
            .with_projection(projection)
            .build(file)
            .unwrap();

        let mut edge_buf = vec![];
        
        while let Some(r) = reader.next() {
            let r = r.unwrap();
            let from = r.column(0).as_primitive::<UInt32Type>();
            let to = r.column(1).as_primitive::<UInt32Type>();
            let arrays = if r.num_columns() > 2 {
                r.columns()[2..].to_vec()
            }else {
                vec![]
            };
            let data_vec = EDATA::from(arrays, from.len());
            
            // 用par_extend 和 into_par_iter
            edge_buf.extend(data_vec.into_iter().enumerate().map(|(index, data)| {
                Edge {
                    from : from.value(index),
                    to : to.value(index),
                    data : data
                }
            }));
        }

        edge_buf
    }

    fn read_vertex<VDATA>(&self, path : String, option : ReadOption)-> Vec<Vertex<VDATA>>
    where 
        VDATA : FromArrow {
        todo!()
    }
}

impl CsvReader {
    pub fn new() -> Self {
        CsvReader{}
    }
}