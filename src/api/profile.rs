use crate::db::profile::UserProfile;

pub fn get() -> UserProfile {
    UserProfile::read()
}

pub fn add_directory(directory: &str) {
    let mut profile = get();
    profile.add(directory);
    profile.save();
}
