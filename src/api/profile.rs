use crate::db::backup::BackupConfig;
use crate::db::profile::{
    DirectoryConfig,
    UserProfile,
};

use uuid::Uuid;

pub fn get() -> UserProfile {
    UserProfile::read()
}

pub fn add_directory(path: String, backup_config: BackupConfig) {
    let mut profile = get();
    profile.add(DirectoryConfig {
        id: Uuid::new_v4().to_string(),
        path,
        backup_config,
    });
    profile.save();
}
