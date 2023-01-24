use crate::api::profile;
use crate::db::profile::{
    DirectoryConfig,
    UserProfile,
};
use crate::db::state::{
    FileState,
    SystemState,
};

use glob::glob;
use std::collections::HashSet;

pub fn previous() -> SystemState {
    SystemState::read("data")
}

pub fn sync() {
    profile::get().iter()
        .for_each(|directory| sync_directory(directory));
    /*
    let previous_state = previous();
    let current_state = current();

    dbg!(previous_state == current_state);

    let difference = previous_state.difference(&current_state);
    dbg!(&difference);

    let profile = profile::get();

    copy_files(&profile, &difference.added);
    copy_files(&profile, &difference.modified);

    current_state.save();
    */
}

fn sync_directory(directory: &DirectoryConfig) {
    let global_state = directory.backup_config.read_global_state();
    let previous_state = get_previous_state(&directory);
    let current_state = get_current_state(&directory);

    dbg!(&directory);
    dbg!(&global_state);
    dbg!(&previous_state);
    dbg!(&current_state);

    for current in current_state.iter() {
        dbg!(current);

        let global = global_state.get(&current.path);
        let previous = previous_state.get(&current.path);

        if global == previous && previous == Some(current) {
            // Scenario a) No changes to sync
            dbg!("Scenario a)");
        } else if global == previous {
            // Scenario b) A change was made locally and needs to be pushed
            dbg!("Scenario b)");
        } else if previous == Some(current) {
            // Scenario c) A change was made to the backup and needs to be pulled
            dbg!("Scenario c)");
        } else {
            // Scenario d) A change was made to both the backup and locally, leading to drift
            dbg!("Scenario d)");
        }
    }
}

fn get_previous_state(directory: &DirectoryConfig) -> SystemState {
    let previous_state = previous().iter()
        .filter(|file| file.root == directory.path)
        .map(|file| file.clone())
        .collect();
    SystemState::new(previous_state)
}

fn get_current_state(directory: &DirectoryConfig) -> SystemState {
    let root = &directory.path;
    let glob_pattern = format!("{}/**/*", root);
    let current_state = glob(&glob_pattern).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .map(|path| FileState::new(path, root))
        .collect();
    SystemState::new(current_state)
}

fn copy_files(profile: &UserProfile, files: &HashSet<FileState>) {
    for file in files {
        let config = profile.get(&file.root);
        config.backup_config.copy_file(file);
    }
}
