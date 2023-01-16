use bincode;
use serde::{
    Serialize,
    de::DeserializeOwned,
};
use std::fs::File;
use std::io::prelude::*;

pub fn read<T: DeserializeOwned>(file_name: &str, default: T) -> T {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut buffer = Vec::<u8>::new();
            file.read_to_end(&mut buffer).unwrap();
            bincode::deserialize(&buffer).unwrap()
        },
        Err(_) => default,
    }
}

pub fn save<T: Serialize>(file_name: &str, data: &T) {
    let encoded = bincode::serialize(data).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(&encoded).unwrap();
}
