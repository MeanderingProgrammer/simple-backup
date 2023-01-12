use bincode;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::prelude::*;

const FILE: &str = "data/profile.bin";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    directories: Vec<String>,
}

impl UserProfile {
    pub fn add(&mut self, directory: &str) {
        self.directories.push(directory.to_string());
    }

    pub fn iter(&self) -> impl Iterator<Item=&String> + '_ {
        self.directories.iter()
    }

    pub fn read() -> Self {
        match fs::File::open(FILE) {
            Ok(mut file) => {
                let mut buffer = Vec::<u8>::new();
                file.read_to_end(&mut buffer).unwrap();
                bincode::deserialize(&buffer).unwrap()
            },
            Err(_) => Self {
                directories: vec![],
            },
        }
    }

    pub fn save(&self) {
        let encoded = bincode::serialize(self).unwrap();
        let mut file = fs::File::create(FILE).unwrap();
        file.write_all(&encoded).unwrap();
    }
}
