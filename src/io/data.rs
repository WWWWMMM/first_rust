use crate::common::base_structure::*;

pub struct Edgesss {
    from : String,
    to : String,
    data : String,
}

pub struct Edgeuss {
    from : Vid,
    to : String,
    data : String,
}

#[derive(Debug)]
pub struct Schema {
    pub name : String,
    pub data_type : String,
}

#[derive(Debug)]
pub struct Header(pub Vec<Schema>);

impl Header {
    /// 每一个属性用`名称:属性`表示，用`,`分割
    pub fn from(str : Vec<&str>) -> Self{
        println!("origin header: {:?}", str);
        
        Header(str.into_iter().map(|str|{
            let p = str.find(":").expect(&format!("missing ':' in {str}"));
            Schema { name : str[0..p].to_string(), data_type : str[p+1..].to_string()}
        }).collect())
    }
}