use crate::db::state::{
    FileState,
    SystemState,
};

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

    pub fn read_global_state(&self) -> SystemState {
        match self {
            Self::Local(config) => SystemState::read(&config.path),
            Self::AwsS3(config) => panic!("Read AWS State Not Implemented: Config = {:?}", config),
        }
    }

    pub fn save_global_state(&self, state: &SystemState) {
        match self {
            Self::Local(config) => state.save(&config.path),
            Self::AwsS3(config) => panic!("Save AWS State Not Implemented: Config = {:?}", config),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LocalConfig {
    pub path: String,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self { path: String::default() }
    }
}

impl LocalConfig {
    pub fn validate(&self) -> Vec<&str> {
        let mut errors = Vec::new();
        if self.path.is_empty() {
            errors.push("No backup directory provided for local configuration");
        }
        errors
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AwsS3Config {
    pub bucket: String,
    pub key: String,
}

impl Default for AwsS3Config {
    fn default() -> Self {
        Self { bucket: String::default(), key: String::default() }
    }
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
