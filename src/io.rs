use crate::common::base_structure::*;
use flate2::read::GzDecoder;
mod data;

pub trait FileRead {
    fn read_edge(&self, path : String)->Vec<Edgeuus>;
    fn read_vertex(&self, path : String)->Vec<Vertexus>;

}

pub struct IO {

}

// impl FileRead for IO {
//     fn read_edge(&self, path : String)->Vec<Edgeuus> {
//         let mut buf = vec![];
//         let file = std::fs::File::open(path).unwrap();
//         println!("{:?}", file);
//         let d = GzDecoder::new(file);
    
//         let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .from_reader(d);
    
//         for result in rdr.records() {
//             let record: csv::StringRecord = result.unwrap();
//             println!("{:?}", record);
//             buf.push(Edgeuus{
//                 from : record.get(0).unwrap().parse().expect("Failed to parse the number"),
//                 to : record.get(1).unwrap().parse().expect("Failed to parse the number"),
//                 data : record.get(2).unwrap().parse().expect("Failed to parse the number"),
//             });
//         }
//         buf
//     }
//     fn read_vertex(&self, path : String)->Vec<Vertexus> {
//         println!("{:?}", path);
//         vec![]
//     }
// }

// impl IO {
//     fn new() -> Self {
//         IO{}
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_read() {
//         println!("????");
//         let io = IO::new();
//         let edges = io.read_edge(String::from("data/a.csv.gz"));
//         println!("{:?}", edges);
//     }
// }