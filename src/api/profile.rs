use crate::db::profile::{
    DirectoryConfig,
    UserProfile,
};

pub fn get() -> UserProfile {
    UserProfile::read()
}

pub fn add_directory(directory: DirectoryConfig) {
    let mut profile = get();
    profile.add(directory);
    profile.save();
}
