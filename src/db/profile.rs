use crate::db::backup::BackupConfig;
use crate::db::util;

use serde::{Serialize, Deserialize};

const FILE_NAME: &str = ".profile.bin";

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
        util::read("data", FILE_NAME, Self::default())
    }

    pub fn save(&self) {
        util::save("data", FILE_NAME, self);
    }
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            directories: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryConfig {
    pub path: String,
    pub backup_config: BackupConfig,
}
