use crate::backup::backup::BackupConfig;
use crate::db::profile::{
    DirectoryConfig,
    UserProfile,
};

use uuid::Uuid;

pub fn get() -> UserProfile {
    UserProfile::read()
}

pub fn get_by_id(id: &str) -> DirectoryConfig {
    get().get_by_id(id).unwrap().clone()
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

pub fn delete_directory(id: &str) {
    let mut profile = get();
    profile.delete(id);
    profile.save();
}
