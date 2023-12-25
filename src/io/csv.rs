use std::fmt::format;

use super::*;

pub struct CsvReader {
    
}
impl FileRead for CsvReader {
    fn read_edge(&self, path : String, option : ReadOption)->(Vec<Edge<String>>, Header) {
        let mut buf = vec![];
        let file = std::fs::File::open(path).unwrap();
        println!("{:?}", file);
        // let file = GzDecoder::new(file);
    
        let mut rdr = ::csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);


        let headers = Header::from(rdr.headers().unwrap().into_iter().collect());
        let from_index = headers.0.iter().position(|x|{
            x.name.eq(&option.from_column)
        }).expect(&format!("missing '{}' column", option.from_column));
        let to_index = headers.0.iter().position(|x|{
            x.name.eq(&option.to_column)
        }).expect(&format!("missing '{}' column", option.to_column));
        let data_indexs = headers.0.iter().enumerate().filter_map(|(index, x)| {
            if option.include_columns.contains(&x.name) {
                Some(index)
            }else {
                None
            }
        }).collect::<Vec<usize>>();

        for result in rdr.records() {
            let record: ::csv::StringRecord = result.unwrap();
            println!("{:?}", record);
            buf.push(Edge::<String>{
                from : record.get(from_index).unwrap().parse().unwrap(),
                to : record.get(to_index).unwrap().parse().unwrap(),
                data : data_indexs.iter().map(|&x|{
                    record.get(x).unwrap()
                }).collect::<Vec<&str>>().join(",")
            });
        }
        
        (buf, headers)
    }
    fn read_vertex(&self, path : String, option : ReadOption)->(Vec<Vertexus>, Header) {
        todo!()
    }
}

impl CsvReader {
    fn new() -> Self {
        CsvReader{}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read() {
        let io = CsvReader::new();
        let mut readoption = ReadOption::default();
        readoption.include_columns.push("data".into());

        let (edges, header) = io.read_edge(String::from("data/a.csv"), readoption);
        println!("{:?}", header);
        println!("{:?}", edges);
    }
}