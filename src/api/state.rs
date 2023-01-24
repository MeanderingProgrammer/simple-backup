use crate::api::profile;
use crate::db::profile::UserProfile;
use crate::db::state::{
    FileState,
    SystemState,
};

use glob::glob;
use std::collections::HashSet;

pub fn previous() -> SystemState {
    SystemState::read()
}

pub fn sync() {
    let previous_state = previous();
    let current_state = current();

    dbg!(previous_state == current_state);

    let difference = previous_state.difference(&current_state);
    dbg!(&difference);

    let profile = profile::get();

    copy_files(&profile, &difference.added);
    copy_files(&profile, &difference.modified);

    current_state.save();
}

fn current() -> SystemState {
    let profile = profile::get();
    let mut state = SystemState::new();
    profile.iter().for_each(|directory| {
        add_file_states(&mut state, &directory.local_path);
    });
    state
}

fn add_file_states(state: &mut SystemState, root: &str) {
    let glob_pattern = format!("{}/**/*", root);
    glob(&glob_pattern).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .for_each(|path| state.add(FileState::new(path, root)));
}

fn copy_files(profile: &UserProfile, files: &HashSet<FileState>) {
    for file in files {
        let config = profile.get(&file.root);
        config.backup_config.copy_file(file);
    }
}
