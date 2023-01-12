use bincode;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::SystemTime;

const FILE: &str = "data/state.bin";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SystemState {
    file_states: HashSet<FileState>,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            file_states: HashSet::new(),
        }
    }

    pub fn add(&mut self, file_state: FileState) {
        self.file_states.insert(file_state);
    }

    pub fn difference<'a>(&'a self, rhs: &'a Self) -> impl Iterator<Item=&FileState> {
        self.file_states.difference(&rhs.file_states)
    }

    pub fn read() -> Self {
        match fs::File::open(FILE) {
            Ok(mut file) => {
                let mut buffer = Vec::<u8>::new();
                file.read_to_end(&mut buffer).unwrap();
                bincode::deserialize(&buffer).unwrap()
            },
            Err(_) => Self::new(),
        }
    }

    pub fn save(&self) {
        let encoded = bincode::serialize(self).unwrap();
        let mut file = fs::File::create(FILE).unwrap();
        file.write_all(&encoded).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct FileState {
    path: String,
    last_modified: u64,
}

impl FileState {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path.as_path().to_str().unwrap().to_string(),
            last_modified: fs::metadata(path).unwrap()
                .modified().unwrap()
                .duration_since(SystemTime::UNIX_EPOCH).unwrap()
                .as_secs(),
        }
    }
}
