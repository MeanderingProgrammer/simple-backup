use crate::system_state::{
    FileState,
    SystemState,
};
use crate::user_profile::UserProfile;

use glob::glob;

pub fn previous() -> SystemState {
    SystemState::read()
}

pub fn current(profile: &UserProfile) -> SystemState {
    let mut state = SystemState::new();
    profile.iter()
        .for_each(|directory| add_file_states(&mut state, &directory));
    state
}

fn add_file_states(state: &mut SystemState, root: &str) {
    let glob_pattern = format!("{}/**/*", root);
    glob(&glob_pattern).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .for_each(|path| state.add(FileState::new(path)));
}
