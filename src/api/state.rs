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
    dbg!(directory);
    dbg!(get_file_states(&directory.path));
    dbg!(SystemState::read("data"));
}

fn current() -> SystemState {
    let profile = profile::get();
    let mut state = SystemState::new();
    profile.iter()
        .for_each(|directory| {
            let file_states = get_file_states(&directory.path);
            file_states.into_iter().for_each(|file_state| state.add(file_state));
        });
    state
}

fn get_file_states(root: &str) -> Vec<FileState> {
    let glob_pattern = format!("{}/**/*", root);
    glob(&glob_pattern).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .map(|path| FileState::new(path, root))
        .collect()
}

fn copy_files(profile: &UserProfile, files: &HashSet<FileState>) {
    for file in files {
        let config = profile.get(&file.root);
        config.backup_config.copy_file(file);
    }
}
