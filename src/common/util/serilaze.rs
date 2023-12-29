use bincode::{config, Decode, Encode};

pub struct Serilazer {
    config : config::Configuration
}

impl Serilazer {
    pub fn new() -> Self {
        Serilazer { config:config::standard() }
    }

    pub fn encode<T>(&self, val : T) -> Vec<u8> 
    where 
        T : bincode::Encode
    {
        bincode::encode_to_vec(&val, self.config).expect("encode failed!")
    }

    pub fn decode<T>(&self, val : Vec<u8>) -> T 
    where 
        T : bincode::Decode
    {
        let (decoded, _): (T, usize) = bincode::decode_from_slice(&val[..], self.config).unwrap();
        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Encode, Decode, PartialEq, Debug)]
    struct Entity <T>{
        x: T,
        y: T,
        // k: String,
    }
    #[test]
    fn test_encode_decode() {
        let serilazer: Serilazer = Serilazer::new();
        let msg: Vec<Entity<f64>>= vec![Entity { x: 0.0, y: 4.0 /*, k : String::from("ff") */}, Entity { x: 10.0, y: 20.5/* , k : String::from("ggg")*/}];
        let encoded: Vec<u8> = serilazer.encode(&msg);
        let decoded : Vec<Entity<f64>> = serilazer.decode(encoded);
        assert_eq!(msg, decoded);
    }
}