use crate::api::profile;
use crate::db::state::{
    FileState,
    SystemState,
};

use glob::glob;

pub fn previous() -> SystemState {
    SystemState::read()
}

pub fn current() -> SystemState {
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
