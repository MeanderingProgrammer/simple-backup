use crate::db::util;

use chrono::{TimeZone, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fs;
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

    pub fn difference<'a>(&'a self, rhs: &'a Self) -> StateDifference {
        let unchanged: HashSet<FileState> = self.file_states.intersection(&rhs.file_states)
            .map(|state| state.clone())
            .collect();

        let current_paths = self.paths();
        let added: HashSet<FileState> = rhs.iter()
            .filter(|state| !current_paths.contains(&state.path))
            .map(|state| state.clone())
            .collect();

        let new_paths = rhs.paths();
        let deleted: HashSet<FileState> = self.iter()
            .filter(|state| !new_paths.contains(&state.path))
            .map(|state| state.clone())
            .collect();

        let modified = current_paths.intersection(&new_paths)
            .map(|path| (self.get(path), rhs.get(path)))
            .filter(|(previous, current)| {
                if previous.last_modified != current.last_modified {
                    assert!(previous.last_modified < current.last_modified, "Only newer files are expected");
                    true
                } else {
                    false
                }
            })
            .map(|(_, current)| current)
            .collect();

        StateDifference {
            unchanged,
            added,
            deleted,
            modified,
        }
    }

    pub fn get(&self, path: &str) -> FileState {
        self.iter().find(|state| state.path == path).unwrap().clone()
    }

    pub fn paths(&self) -> HashSet<String> {
        self.iter()
            .map(|state| state.path.clone())
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item=&FileState> {
        self.file_states.iter()
    }

    pub fn read() -> Self {
        util::read(FILE, Self::new())
    }

    pub fn save(&self) {
        util::save(FILE, self);
    }
}

#[derive(Debug)]
pub struct StateDifference {
    pub unchanged: HashSet<FileState>,
    pub added: HashSet<FileState>,
    pub deleted: HashSet<FileState>,
    pub modified: HashSet<FileState>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct FileState {
    pub path: String,
    pub root: String,
    pub suffix: String,
    pub last_modified: u64,
}

impl FileState {
    pub fn new(path: PathBuf, root: &str) -> Self {
        let path_str = path.as_path().to_str().unwrap().to_string();
        let suffix = path_str.strip_prefix(root).unwrap();
        Self {
            path: path_str.clone(),
            root: root.to_string(),
            suffix: suffix.to_string(),
            last_modified: fs::metadata(path).unwrap()
                .modified().unwrap()
                .duration_since(SystemTime::UNIX_EPOCH).unwrap()
                .as_secs(),
        }
    }

    pub fn to_date(&self, date_format: &str) -> String {
        let time_in_seconds = self.last_modified.try_into().unwrap();
        let date_time = Utc.timestamp_opt(time_in_seconds, 0).single().unwrap();
        date_time.format(date_format).to_string()
    }
}
