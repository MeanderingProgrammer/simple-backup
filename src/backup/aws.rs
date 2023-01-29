use crate::db::state::FileState;

use serde::{Serialize, Deserialize};

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

    pub fn exists(&self) -> bool {
        panic!("AWS State Exists Not Implemented: Config = {:?}", self);
    }

    pub fn push(&self, _file: &FileState) {
        panic!("Push AWS State Not Implemented: Config = {:?}", self);
    }

    pub fn pull(&self, _file: &FileState) {
        panic!("Pull AWS State Not Implemented: Config = {:?}", self);
    }
}
