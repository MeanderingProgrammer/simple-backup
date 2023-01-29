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
    //final_state.save("data");
}

fn sync_directory(directory: &DirectoryConfig) -> SystemState {
    dbg!(directory);

    let previous_state = SystemState::new(get_previous(directory));
    dbg!(&previous_state);

    if directory.backup_config.exists() {
        let global_state = directory.backup_config.read_global_state();
        let current_state = SystemState::new(get_current(directory));

        let state_manager = StateManager::new(
            directory,
            &global_state,
            &previous_state,
            &current_state,
        );
        state_manager.sync_directory();

        // At this point the current state is our source of truth, however we need to pull it again
        // first as it may have changed due to retrieving data from the global state

        // TODO - modify current state on retrieval from global state instead
        let synced_current_state = SystemState::new(get_current(&directory));
        //directory.backup_config.save_global_state(&synced_current_state);
        synced_current_state
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
