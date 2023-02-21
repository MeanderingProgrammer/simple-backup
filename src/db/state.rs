use crate::db::util;

use chrono::{TimeZone, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

const FILE_NAME: &str = ".state.bin";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SystemState {
    file_states: Vec<FileState>,
}

impl SystemState {
    pub fn new(file_states: Vec<FileState>) -> Self {
        Self {
            file_states,
        }
    }

    pub fn add(&mut self, file_state: FileState) {
        self.file_states.push(file_state);
    }

    pub fn get(&self, path: &str) -> Option<&FileState> {
        self.iter().find(|state| state.path == path)
    }

    pub fn files_paths(&self) -> HashSet<String> {
        self.iter()
            .map(|state| state.path.clone())
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item=&FileState> {
        self.file_states.iter()
    }

    pub fn read(root: &str) -> Self {
        util::read(root, FILE_NAME, Self::default())
    }

    pub fn save(&self, root: &str) {
        util::save(root, FILE_NAME, self);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileState {
    pub owner_id: String,
    pub root: String,
    pub path: String,
    pub suffix: String,
    pub last_modified: u64,
}

impl FileState {
    pub fn new(owner_id: &str, root: &str, path: PathBuf) -> Self {
        let path_str = path.as_path().to_str().unwrap().to_string();
        let suffix = path_str.strip_prefix(root).unwrap();

        Self {
            owner_id: owner_id.to_string(),
            root: root.to_string(),
            path: path_str.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OWNER_ID: &str = "42135de4-c71f-4097-a7ba-7551eadca921";
    const TEST_WINDOWS_ROOT: &str = "C:\\Users\\vsusl\\Documents\\scripts\\backup-test\\local";

    #[test]
    fn it_works() {
        let state = FileState::new(
            TEST_OWNER_ID,
            TEST_WINDOWS_ROOT,
            PathBuf::from(format!("{}\\folder-1\\file-1.txt", TEST_WINDOWS_ROOT)),
        );
        println!("{:#?}", state);
        assert_eq!(4, 4);
    }
}
