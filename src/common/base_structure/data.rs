use std::{ops::{Div, Mul, Sub, Add}, collections::HashMap};

use bincode::Encode;

struct Empty;

pub struct Data {
    data : Vec<DataUnit>
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Encode)]
pub enum DataUnit {
    EMPTY,
    BOOL(bool),
    INT(i32),
    LONG(i64),
    FLOAT(f32),
    DOUBLE(f64),
    STRING(String),
}

impl Add for &DataUnit {
    type Output = DataUnit;

    fn add(self, other: &DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a + b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a + b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a + b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a + b),
            (_, _) => unreachable!()
        }
    }
}

impl Add for DataUnit {
    type Output = DataUnit;

    fn add(self, other: DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a + b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a + b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a + b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a + b),
            (_, _) => unreachable!()
        }
    }
}

impl Sub for &DataUnit {
    type Output = DataUnit;

    fn sub(self, other: &DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a - b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a - b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a - b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a - b),
            (_, _) => unreachable!()
        }
    }
}

impl Sub for DataUnit {
    type Output = DataUnit;

    fn sub(self, other: DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a - b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a - b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a - b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a - b),
            (_, _) => unreachable!()
        }
    }
}

impl Mul for &DataUnit {
    type Output = DataUnit;

    fn mul(self, other: &DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a * b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a * b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a * b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a * b),
            (_, _) => unreachable!()
        }
    }
}

impl Mul for DataUnit {
    type Output = DataUnit;

    fn mul(self, other: DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a * b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a * b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a * b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a * b),
            (_, _) => unreachable!()
        }
    }
}

impl Div for &DataUnit {
    type Output = DataUnit;

    fn div(self, other: &DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a / b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a / b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a / b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a / b),
            (_, _) => unreachable!()
        }
    }
}

impl Div for DataUnit {
    type Output = DataUnit;

    fn div(self, other: DataUnit) -> DataUnit {
        match (self, other) {
            (DataUnit::INT(a), DataUnit::INT(b)) => DataUnit::INT(a / b),
            (DataUnit::LONG(a), DataUnit::LONG(b)) => DataUnit::LONG(a / b),
            (DataUnit::FLOAT(a), DataUnit::FLOAT(b)) => DataUnit::FLOAT(a / b),
            (DataUnit::DOUBLE(a), DataUnit::DOUBLE(b)) => DataUnit::DOUBLE(a / b),
            (_, _) => unreachable!()
        }
    }
}

impl DataUnit {
    /// 获得字符串所表达的类型，返回值用于 `from` 和 `create_judge` 函数的第二个参数。
    fn get_type(data_type : &str) -> Self {
        match data_type {
            "" | "empty" => DataUnit::EMPTY,
            "int" | "int32" => DataUnit::INT(0),
            "long" | "int64" | "long long" => DataUnit::LONG(0),
            "float" | "float32" => DataUnit::FLOAT(0.0),
            "double" | "float64" => DataUnit::DOUBLE(0.0),
            "string" => DataUnit::STRING(String::default()),
            _ => unreachable!()
        }
    }

    /// 将字符串数据解析成特定类型的数据。
    fn from(data : &str, data_type : &Self) -> Self {
        match data_type {
            DataUnit::EMPTY => DataUnit::EMPTY,
            DataUnit::BOOL(_) => DataUnit::BOOL(data.parse().unwrap()),
            DataUnit::INT(_) => DataUnit::INT(data.parse().unwrap()),
            DataUnit::LONG(_) => DataUnit::LONG(data.parse().unwrap()),
            DataUnit::FLOAT(_) => DataUnit::FLOAT(data.parse().unwrap()),
            DataUnit::DOUBLE(_) => DataUnit::DOUBLE(data.parse().unwrap()),
            DataUnit::STRING(_) => DataUnit::STRING(data.parse().unwrap())
        }
    }

    /// 返回用于判断 'Data' 是否满足特定限制的闭包。
    fn create_judge(constrain: &str, data_type : Self) -> Box<dyn Fn(&DataUnit) -> bool> {
        
        let closure = move |x: &DataUnit| -> bool {
            // 在闭包的逻辑中使用 string_value 和 x
            // 返回一个布尔值作为结果
            false
        };
    
        Box::new(closure)
    }
}

/// 数据类型必须完全相同
fn convert_to<T>(data: DataUnit) -> T
where
    T: std::convert::From<DataUnit>,
{
    T::from(data)
}

impl std::convert::From<DataUnit> for Empty {
    fn from(data: DataUnit) -> Self {
        match data {
            DataUnit::EMPTY => Empty,
            _ => panic!("Invalid conversion to Empty"),
        }
    }
}

impl std::convert::From<DataUnit> for bool {
    fn from(data: DataUnit) -> Self {
        match data {
            DataUnit::BOOL(value) => value,
            _ => panic!("Invalid conversion to bool"),
        }
    }
}

impl std::convert::From<DataUnit> for i32 {
    fn from(data: DataUnit) -> Self {
        match data {
            DataUnit::INT(value) => value,
            _ => panic!("Invalid conversion to i32"),
        }
    }
}

impl std::convert::From<DataUnit> for i64 {
    fn from(data: DataUnit) -> Self {
        match data {
            DataUnit::LONG(value) => value,
            _ => panic!("Invalid conversion to i64"),
        }
    }
}

impl std::convert::From<DataUnit> for f32 {
    fn from(data: DataUnit) -> Self {
        match data {
            DataUnit::FLOAT(value) => value,
            _ => panic!("Invalid conversion to f32"),
        }
    }
}

impl std::convert::From<DataUnit> for f64 {
    fn from(data: DataUnit) -> Self {
        match data {
            DataUnit::DOUBLE(value) => value,
            _ => panic!("Invalid conversion to f64"),
        }
    }
}

impl std::convert::From<DataUnit> for String {
    fn from(data: DataUnit) -> Self {
        match data {
            DataUnit::STRING(value) => value,
            _ => panic!("Invalid conversion to f64"),
        }
    }
}

struct OperationBuilder {
    schema : HashMap<String, usize>
}

impl OperationBuilder {
    fn new() -> Self {
        todo!()
    }

    fn judge1(str : &str) -> Box<dyn Fn(&Data) -> bool> {
        todo!()
    }

    fn judge2(str : &str) -> Box<dyn Fn(&Data, &Data) -> bool> {
        todo!()
    }

    fn judge3(str : &str) -> Box<dyn Fn(&Data, &Data, &Data) -> bool> {
        todo!()
    }

    fn combine1(str : &str) -> Box<dyn Fn(&Data) -> Data> {
        todo!()
    }

    fn combine2(str : &str) -> Box<dyn Fn(&Data, &Data) -> Data> {
        todo!()
    }

    fn combine3(str : &str) -> Box<dyn Fn(&Data, &Data, &Data) -> Data> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Serilazer;

    #[test]
    fn convert_dataunit() {
        let data = DataUnit::FLOAT(3.14);
        let converted_value: f32 = convert_to(data);
        

        assert_eq!(3.14, converted_value);
    }

    #[test]
    fn data_encode() {
        let s = Serilazer::new();
        let a = DataUnit::EMPTY;
        let val = s.encode(a);
        println!("{:?}", val.len());
        // assert_eq!(val.len(), 0);

        let a = DataUnit::BOOL(false);
        let val = s.encode(a);
        println!("{:?}", val.len());

        let a = DataUnit::INT(0);
        let val = s.encode(a);
        println!("{:?}", val.len());

        let a = 0;
        let val = s.encode(a);
        println!("{:?}", val.len());
    }
}