use crate::backup::aws::AwsS3Config;
use crate::backup::local::LocalConfig;
use crate::db::state::{
    FileState,
    SystemState,
};

use serde::{Serialize, Deserialize};

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
            Self::AwsS3(config) => config.exists(),
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
            Self::AwsS3(config) => config.push(file),
        };
    }

    pub fn pull(&self, file: &FileState) {
        match self {
            Self::Local(config) => config.pull(file),
            Self::AwsS3(config) => config.pull(file),
        };
    }

    pub fn save_backup_state(&self, state: &SystemState) {
        match self {
            Self::Local(config) => state.save(&config.path),
            Self::AwsS3(config) => panic!("Save AWS State Not Implemented: Config = {:?}", config),
        };
    }
}
