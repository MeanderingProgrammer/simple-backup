use crate::backup::backup::BackupConfig;
use crate::db::util;

use serde::{Serialize, Deserialize};

const FILE_NAME: &str = ".profile.bin";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserProfile {
    directories: Vec<DirectoryConfig>,
}

impl UserProfile {
    pub fn add(&mut self, directory: DirectoryConfig) {
        self.directories.push(directory);
    }

    pub fn delete(&mut self, id: &str) {
        let index = self.iter().position(|directory| directory.id == id).unwrap();
        self.directories.remove(index);
    }

    pub fn iter(&self) -> impl Iterator<Item=&DirectoryConfig> {
        self.directories.iter()
    }

    pub fn get_by_id(&self, id: &str) -> Option<&DirectoryConfig> {
        self.iter().find(|directory| directory.id == id)
    }

    pub fn read() -> Self {
        util::read("data", FILE_NAME, Self::default())
    }

    pub fn save(&self) {
        util::save("data", FILE_NAME, self);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectoryConfig {
    pub id: String,
    pub path: String,
    pub backup_config: BackupConfig,
}
