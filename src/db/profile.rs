use crate::db::util;

use serde::{Serialize, Deserialize};

const FILE: &str = "data/profile.bin";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    directories: Vec<DirectoryConfig>,
}

impl UserProfile {
    pub fn add(&mut self, directory: DirectoryConfig) {
        self.directories.push(directory);
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BackupConfig {
    Local(LocalConfig),
    AwsS3(AwsS3Config),
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self::Local(LocalConfig::default())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LocalConfig {
    pub path: String,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self { path: String::default() }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AwsS3Config {
    pub bucket: String,
    pub key: String,
}

impl Default for AwsS3Config {
    fn default() -> Self {
        Self { bucket: String::default(), key: String::default() }
    }
}
