use crate::db::state::{
    FileState,
    SystemState,
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

    pub fn push(&self, file: &FileState) {
        match self {
            Self::Local(config) => config.push(file),
            Self::AwsS3(config) => config.push(file),
        };
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

        let to_path = format!("{}{}", &self.path, &file.suffix);
        let to_location = Path::new(&to_path);

        // Create the file if it does not already exist, before starting the copy
        if !to_location.exists() {
            // Create the directory structure if nedded
            let directory = to_location.parent().unwrap();
            if !directory.exists() {
                fs::create_dir_all(directory).unwrap();
            }
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(to_location)
                .unwrap();
        }

        fs::copy(from_location, to_location).unwrap();
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

    pub fn push(&self, file: &FileState) {
        dbg!(file);
        println!("TODO");
    }
}
