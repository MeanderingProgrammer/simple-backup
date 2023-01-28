use crate::db::backup::BackupConfig;
use crate::db::profile::DirectoryConfig;
use crate::db::state::{
    FileState,
    SystemState,
};

use filetime::FileTime;
use std::collections::HashSet;

#[derive(Debug)]
pub struct StateManager<'a> {
    directory: &'a DirectoryConfig,
    backup_config: &'a BackupConfig,
    global_state: &'a SystemState,
    previous_state: &'a SystemState,
    current_state: &'a SystemState,
}

impl<'a> StateManager<'a> {
    pub fn new(
        directory: &'a DirectoryConfig,
        global_state: &'a SystemState,
        previous_state: &'a SystemState,
        current_state: &'a SystemState,
    ) -> Self {
        Self {
            directory,
            backup_config: &directory.backup_config,
            global_state,
            previous_state,
            current_state,
        }
    }

    pub fn sync_directory(&self) {
        self.get_file_paths().iter()
            .for_each(|file_path| self.sync_file(file_path));
    }

    fn get_file_paths(&self) -> HashSet<String> {
        let mut file_paths = self.global_state.files_paths();
        file_paths.extend(self.previous_state.files_paths());
        file_paths.extend(self.current_state.files_paths());
        file_paths
    }

    fn sync_file(&self, file_path: &String) {
        dbg!(file_path);

        let global = self.global_state.get(file_path);
        let previous = self.previous_state.get(file_path);
        let current = self.current_state.get(file_path);

        match (global, previous, current) {
            (Some(global_file), Some(previous_file), Some(current_file)) => println!("ALL EXIST"),

            (None, Some(previous_file), Some(current_file)) => println!("NOT GLOBALLY TRACKED"),
            (Some(global_file), None, Some(current_file)) => println!("NEW TO CURRENT COMPUTER"),
            (Some(global_file), Some(previous_file), None) => println!("REMOVED FROM CURRENT COMPUTER"),

            (None, None, Some(current_file)) => println!("ONLY ON CURRENT"),
            (None, Some(previous_file), None) => println!("ONLY ON PREVIOUS"),
            (Some(global_file), None, None) => println!("ONLY ON GLOBAL"),

            (None, None, None) => panic!("Attempting to sync a file not being tracked anywhere"),
        };

        // 1. Previous is ahead of current, this should never happen, and we'll
        //    assume it never does.
        //
        // 2. Current is ahead of previous
        let local_requires_sync = previous != current;

        // 1. Global is behind previous, this can happen if we try to sync with AWS
        //    without internet, in which case previous will still be updated.
        //   * Ideally Response: overwrite the global state when possible
        //
        // 2. Global is ahead of previous, this can happen if we sync on a separate
        //    comuter before syncing on this one.
        //   * Ideally Response: overwrite the local state
        let sync_with_backup_required = global != previous;

        match (local_requires_sync, sync_with_backup_required) {
            (false, false) => {
                // a) No changes to sync, backup and local haven't changed
                dbg!(&file_path, "hasn't changed");
            },
            (true, false) => {
                // b) A change was made locally and needs to be pushed
                //    Backup is unchanged so nothing to pull down
                dbg!(&file_path, "pushing to backup", self.backup_config);
                self.backup_config.push(current.unwrap());
            },
            (false, true) => {
                // c) A change was made to the backup and needs to be pulled
                //    Local is unchanged so nothin need to be pushed
                dbg!(&file_path, "pulling from backup");
                pull_from_global(global);
           },
            (true, true) => {
                // d) A change was made to both the backup and locally, leading to drift
                panic!("{} has changed both locally and in backup, for now we crash", &file_path);
            },
        };
    }
}

fn pull_from_global(global: Option<&FileState>) {
    // Will need to synchronize the modified time to match global state
    // Use set_file_mtime method to update this value:
    //  * https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    // Use from_unix_time to generate the correct value:
    //  * https://docs.rs/filetime/latest/filetime/struct.FileTime.html#method.from_unix_time
    dbg!(global.unwrap().last_modified);
    dbg!(global.unwrap().last_modified as i64);
    let time_to_set = FileTime::from_unix_time(global.unwrap().last_modified as i64, 0);
    dbg!(time_to_set);
}
