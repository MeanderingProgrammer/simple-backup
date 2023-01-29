use crate::api::profile;
use crate::db::profile::DirectoryConfig;
use crate::db::state::{
    FileState,
    SystemState,
};
use crate::manager::state::StateManager;

use glob::glob;

pub fn previous() -> SystemState {
    SystemState::read("data")
}

pub fn sync() {
    let mut final_state = SystemState::default();
    profile::get().iter().for_each(|directory| {
        sync_directory(directory).iter().for_each(|file| {
            final_state.add(file.clone());
        });
    });
    final_state.save("data");
}

fn sync_directory(directory: &DirectoryConfig) -> SystemState {
    let previous_state = SystemState::new(get_previous(directory));
    if directory.backup_config.exists() {
        let backup_state = directory.backup_config.read_backup_state();
        let current_state = SystemState::new(get_current(directory));

        let synced_state = StateManager::new(directory, &backup_state, &previous_state, &current_state).sync_directory();
        directory.backup_config.save_backup_state(&synced_state);
        synced_state
    } else {
        println!("BACKUP NOT CONNECTED: RE-USING PREVIOUS STATE");
        previous_state
    }
}

fn get_previous(directory: &DirectoryConfig) -> Vec<FileState> {
    previous().iter()
        .filter(|state| state.owner_id == directory.id)
        .map(|state| state.clone())
        .collect()
}

fn get_current(directory: &DirectoryConfig) -> Vec<FileState> {
    let glob_pattern = format!("{}/**/*", directory.path);
    glob(&glob_pattern).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .map(|path| FileState::new(path, directory))
        .collect()
}
