use crate::db::backup::BackupConfig;
use crate::db::profile::DirectoryConfig;
use crate::db::state::{
    FileState,
    SystemState,
};

use filetime::FileTime;
use std::collections::HashSet;

/**
 * StateManager assumes that a connection to the backup exists
 *
 * This means that if backup state returns an empty state for a file then it does not exist,
 * as opposed to, for example, not being able to connect AWS to pull the backup state.
 *
 * An entirely empty backup state means this is the first time we're syncing.
 */

#[derive(Debug)]
pub struct StateManager<'a> {
    backup_config: &'a BackupConfig,
    backup_state: &'a SystemState,
    previous_state: &'a SystemState,
    current_state: &'a SystemState,
}

impl<'a> StateManager<'a> {
    pub fn new(
        directory: &'a DirectoryConfig,
        backup_state: &'a SystemState,
        previous_state: &'a SystemState,
        current_state: &'a SystemState,
    ) -> Self {
        Self {
            backup_config: &directory.backup_config,
            backup_state,
            previous_state,
            current_state,
        }
    }

    pub fn sync_directory(&self) {
        self.get_file_paths().iter()
            .for_each(|file_path| self.sync_file(file_path));
    }

    fn get_file_paths(&self) -> HashSet<String> {
        let mut file_paths = self.backup_state.files_paths();
        file_paths.extend(self.previous_state.files_paths());
        file_paths.extend(self.current_state.files_paths());
        file_paths
    }

    fn sync_file(&self, file_path: &String) {
        match (self.backup_state.get(file_path), self.previous_state.get(file_path), self.current_state.get(file_path)) {
            (Some(backup_file), Some(previous_file), Some(current_file)) => {
                let local_changed = previous_file != current_file;
                let backup_changed = backup_file != previous_file;
                match (local_changed, backup_changed) {
                    (false, false) => println!("{} has not changed, nothing to do", &file_path),
                    (true, false) => self.push_to_backup(current_file),
                    (false, true) => pull_from_backup(backup_file),
                    (true, true) => panic!("{} has changed both locally and in backup, for now we crash", &file_path),
                };
            },

            (None, _, Some(current_file)) => self.push_to_backup(current_file),
            (Some(backup_file), _, None) => pull_from_backup(backup_file),

            (Some(_), None, Some(_)) => panic!("File exists locally but was not pulled down correctly"),
            (None, Some(_), None) => println!("File was removed locally and from backup, nothing to do"),
            (None, None, None) => panic!("Attempting to sync a file not being tracked anywhere, should not be possible"),
        };
    }

    fn push_to_backup(&self, current_file: &FileState) {
        self.backup_config.push(current_file);
    }
}

fn pull_from_backup(backup_file: &FileState) {
    // Will need to synchronize the modified time to match global state
    // Use set_file_mtime method to update this value:
    //  * https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    // Use from_unix_time to generate the correct value:
    //  * https://docs.rs/filetime/latest/filetime/struct.FileTime.html#method.from_unix_time
    dbg!(backup_file.last_modified);
    dbg!(backup_file.last_modified as i64);
    let time_to_set = FileTime::from_unix_time(backup_file.last_modified as i64, 0);
    dbg!(time_to_set);
}
