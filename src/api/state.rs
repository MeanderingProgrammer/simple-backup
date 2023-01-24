use crate::api::profile;
use crate::db::profile::DirectoryConfig;
use crate::db::state::{
    FileState,
    SystemState,
};

use glob::glob;

pub fn previous() -> SystemState {
    SystemState::read("data")
}

pub fn sync() {
    let mut final_state = SystemState::default();
    profile::get().iter()
        .for_each(|directory| {
            sync_directory(directory).iter()
                .for_each(|file| final_state.add(file.clone()));
        });
    //final_state.save("data");
}

fn sync_directory(directory: &DirectoryConfig) -> SystemState {
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
            directory.backup_config.copy_file(current);
        } else if previous == Some(current) {
            // Scenario c) A change was made to the backup and needs to be pulled
            dbg!("Scenario c)");
        } else {
            // Scenario d) A change was made to both the backup and locally, leading to drift
            dbg!("Scenario d)");
        }
    }

    // At this point the current state is our source of truth, however we need to pull it again
    // first as it may have changed due to retrieving data from the global state
    // TODO - modify current state on retrieval from global state instead

    let synced_current_state = get_current_state(&directory);
    //directory.backup_config.save_global_state(&synced_current_state);
    synced_current_state
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
