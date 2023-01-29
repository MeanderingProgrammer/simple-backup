use crate::db::state::{
    FileState,
    SystemState,
};
use crate::db::util;

use filetime::{
    FileTime,
    set_file_mtime,
};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BackupConfig {
    Local(LocalConfig),
    AwsS3(AwsS3Config),
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self::Local(LocalConfig::default())
    }
}

impl BackupConfig {
    pub fn validate(&self) -> Vec<&str> {
        match self {
            Self::Local(config) => config.validate(),
            Self::AwsS3(config) => config.validate(),
        }
    }

    pub fn exists(&self) -> bool {
        match self {
            Self::Local(config) => config.exists(),
            Self::AwsS3(config) => panic!("AWS State Exists Not Implemented: Config = {:?}", config),
        }
    }

    pub fn read_backup_state(&self) -> SystemState {
        match self {
            Self::Local(config) => SystemState::read(&config.path),
            Self::AwsS3(config) => panic!("Read AWS State Not Implemented: Config = {:?}", config),
        }
    }

    pub fn push(&self, file: &FileState) {
        match self {
            Self::Local(config) => config.push(file),
            Self::AwsS3(config) => panic!("Push AWS State Not Implemented: Config = {:?}", config),
        };
    }

    pub fn pull(&self, file: &FileState) {
        match self {
            Self::Local(config) => config.pull(file),
            Self::AwsS3(config) => panic!("Pull AWS State Not Implemented: Config = {:?}", config),
        };
    }

    pub fn save_backup_state(&self, state: &SystemState) {
        match self {
            Self::Local(config) => state.save(&config.path),
            Self::AwsS3(config) => panic!("Save AWS State Not Implemented: Config = {:?}", config),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct LocalConfig {
    pub path: String,
}

impl LocalConfig {
    pub fn validate(&self) -> Vec<&str> {
        let mut errors = Vec::new();
        if self.path.is_empty() {
            errors.push("No backup directory provided for local configuration");
        }
        errors
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub fn push(&self, file: &FileState) {
        let from_location = Path::new(&file.path);

        let to_path = self.backup_path(file);
        let to_location = Path::new(&to_path);

        // Create the file if it does not already exist, before starting the copy
        util::create_file_at_path(&to_location);

        fs::copy(from_location, to_location).unwrap();
    }

    pub fn pull(&self, file: &FileState) {
        let from_path = self.backup_path(file);
        let from_location = Path::new(&from_path);

        let to_location = Path::new(&file.path);

        // Create the file if it does not already exist, before starting the copy
        util::create_file_at_path(&to_location);

        fs::copy(from_location, to_location).unwrap();

        // Handles synchronizing the modified time to match global state
        let time_to_set = FileTime::from_unix_time(file.last_modified as i64, 0);
        set_file_mtime(to_location, time_to_set).unwrap();
    }

    fn backup_path(&self, file: &FileState) -> String {
        format!("{}{}", &self.path, &file.suffix)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AwsS3Config {
    pub bucket: String,
    pub key: String,
}

impl AwsS3Config {
    pub fn validate(&self) -> Vec<&str> {
        let mut errors = Vec::new();
        if self.bucket.is_empty() {
            errors.push("No bucket provided for aws configuration");
        }
        if self.key.is_empty() {
            errors.push("No key provided for aws configuration");
        }
        errors
    }
}
