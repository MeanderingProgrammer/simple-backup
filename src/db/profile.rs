use crate::db::{
    state::FileState,
    util,
};

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const FILE: &str = "data/profile.bin";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    directories: Vec<DirectoryConfig>,
}

impl UserProfile {
    pub fn add(&mut self, directory: DirectoryConfig) {
        self.directories.push(directory);
    }

    pub fn get(&self, path: &str) -> &DirectoryConfig {
        self.iter().find(|directory| directory.local_path == path).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item=&DirectoryConfig> {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryConfig {
    pub local_path: String,
    pub backup_config: BackupConfig,
}

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

    pub fn copy_file(&self, file: &FileState) {
        match self {
            Self::Local(config) => config.copy_file(file),
            Self::AwsS3(config) => config.copy_file(file),
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

    pub fn copy_file(&self, file: &FileState) {
        dbg!(self);
        dbg!(file);

        let from_location = Path::new(&file.path);

        let to_path = format!("{}{}", &self.path, &file.suffix);
        let to_location = Path::new(&to_path);

        // Create the file if it does not already exist, before starting the copy
        if !to_location.exists() {
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(to_location)
                .unwrap();
        }

        dbg!(from_location);
        dbg!(to_location.exists());

        dbg!(from_location.parent());
        dbg!(to_location.parent());
        //let result = fs::copy(from_location, to_location);
        //dbg!(result);
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

    pub fn copy_file(&self, file: &FileState) {
        dbg!(file);
        println!("TODO");
    }
}
