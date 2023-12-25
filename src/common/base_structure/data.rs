use std::ops::{Div, Mul, Sub, Add};

#[derive(Debug, PartialEq, Clone)]
pub struct Data {

}

impl Add for Data {
    type Output = Data;

    fn add(self, other: Data) -> Data {
        Data {
            
        }
    }
}

impl Sub for Data {
    type Output = Data;

    fn sub(self, other: Data) -> Data {
        Data {
            
        }
    }
}

impl Mul for Data {
    type Output = Data;

    fn mul(self, other: Data) -> Data {
        Data {
            
        }
    }
}

impl Div for Data {
    type Output = Data;

    fn div(self, other: Data) -> Data {
        Data {
            
        }
    }
}

impl Data {
    fn new(content : String, constrain : String) -> Self {
        Data {}
    }
}


fn create_judge(constrain: String) -> Box<dyn Fn(&Data) -> bool> {
    let closure = move |x: &Data| -> bool {
        // 在闭包的逻辑中使用 string_value 和 x
        // 返回一个布尔值作为结果
        false
    };

    Box::new(closure)
}

fn create_closure_vec(string_value: String) -> Vec<Box<dyn Fn(i32) -> bool>> {
    let closure = move |x: i32| -> bool {
        // 在闭包的逻辑中使用 string_value 和 x
        // 返回一个布尔值作为结果
        false
    };

    vec![Box::new(closure)]
}