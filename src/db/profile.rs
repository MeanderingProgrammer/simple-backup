use crate::db::util;

use serde::{Serialize, Deserialize};

const FILE: &str = "data/profile.bin";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    directories: Vec<String>,
}

impl UserProfile {
    pub fn add(&mut self, directory: &str) {
        self.directories.push(directory.to_string());
    }

    pub fn iter(&self) -> impl Iterator<Item=&String> {
        self.directories.iter()
    }

    pub fn read() -> Self {
        util::read(FILE, Self {
            directories: vec![],
        })
    }

    pub fn save(&self) {
        util::save(FILE, self);
    }
}
