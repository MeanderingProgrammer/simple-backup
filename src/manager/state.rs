use crate::backup::backup::BackupConfig;
use crate::db::profile::DirectoryConfig;
use crate::db::state::{
    FileState,
    SystemState,
};

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

    pub fn sync_directory(&self) -> SystemState {
        let mut synced_state = SystemState::default();
        self.get_file_paths().iter().for_each(|file_path| {
            dbg!(file_path);
            match self.sync_file(file_path) {
                Some(file_state) => synced_state.add(file_state),
                None => println!("No state assoicated with file"),
            };
        });
        synced_state
    }

    fn get_file_paths(&self) -> HashSet<String> {
        let mut file_paths = self.backup_state.files_paths();
        file_paths.extend(self.previous_state.files_paths());
        file_paths.extend(self.current_state.files_paths());
        file_paths
    }

    fn sync_file(&self, file_path: &str) -> Option<FileState> {
        match self.get_states(file_path) {
            (Some(backup_file), Some(previous_file), Some(current_file)) => {
                let local_changed = previous_file != current_file;
                let backup_changed = previous_file != backup_file;
                match (local_changed, backup_changed) {
                    (false, false) => self.unchanged_file(file_path, current_file),
                    (true, false) => self.push_to_backup(file_path, current_file),
                    (false, true) => self.pull_from_backup(file_path, backup_file),
                    (true, true) => panic!("{file_path}: Has been modified both locally and in backup, for now we crash :("),
                }
            },

            (None, _, Some(current_file)) => self.push_to_backup(file_path, current_file),
            (Some(backup_file), _, None) => self.pull_from_backup(file_path, backup_file),

            (None, Some(_), None) => self.removed_file(file_path),
            (Some(_), None, Some(_)) => panic!("{file_path}: Exists locally but was not pulled down correctly"),
            (None, None, None) => panic!("{file_path}: Attempting to sync a file not being tracked anywhere"),
        }
    }

    fn get_states(&self, file_path: &str) -> (Option<&FileState>, Option<&FileState>, Option<&FileState>) {
        (
            self.backup_state.get(file_path),
            self.previous_state.get(file_path),
            self.current_state.get(file_path),
        )
    }

    fn unchanged_file(&self, file_path: &str, current_file: &FileState) -> Option<FileState> {
        println!("{file_path}: Has not been modified, nothing to do");
        Some(current_file.clone())
    }

    fn removed_file(&self, file_path: &str) -> Option<FileState> {
        println!("{file_path}: Was removed locally and from backup, nothing to do");
        None
    }

    fn push_to_backup(&self, file_path: &str, current_file: &FileState) -> Option<FileState> {
        println!("{file_path}: Has been modified locally, pushing to backup");
        self.backup_config.push(current_file);
        Some(current_file.clone())
    }

    fn pull_from_backup(&self, file_path: &str, backup_file: &FileState) -> Option<FileState> {
        println!("{file_path}: Has been modified in backup, pulling from backup");
        self.backup_config.pull(backup_file);
        Some(backup_file.clone())
    }
}
