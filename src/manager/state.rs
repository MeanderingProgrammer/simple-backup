use crate::backup::backup::BackupConfig;
use crate::db::profile::DirectoryConfig;
use crate::db::state::{
    FileState,
    SystemState,
};

use std::collections::HashSet;

/**
 * SystemStateManager assumes that a connection to the backup exists
 *
 * This means that if backup state returns an empty state for a file then it does not exist,
 * as opposed to, for example, not being able to connect AWS to pull the backup state.
 *
 * An entirely empty backup state means this is the first time we're syncing.
 */

#[derive(Debug)]
pub struct SystemStateManager<'a> {
    backup_config: &'a BackupConfig,
    backup_state: &'a SystemState,
    previous_state: &'a SystemState,
    current_state: &'a SystemState,
}

impl<'a> SystemStateManager<'a> {
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
            let file_state_manager = self.get_file_state_manager(file_path);
            match file_state_manager.sync() {
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

    fn get_file_state_manager(&'a self, file_path: &'a str) -> FileStateManager {
        FileStateManager {
            backup_config: self.backup_config,
            file_path,
            backup_file: self.backup_state.get(file_path),
            previous_file: self.previous_state.get(file_path),
            current_file: self.current_state.get(file_path),
        }
    }
}

#[derive(Debug)]
struct FileStateManager<'a> {
    backup_config: &'a BackupConfig,
    file_path: &'a str,
    backup_file: Option<&'a FileState>,
    previous_file: Option<&'a FileState>,
    current_file: Option<&'a FileState>,
}

impl<'a> FileStateManager<'a> {
    fn sync(&self) -> Option<FileState> {
        match (self.backup_file, self.previous_file, self.current_file) {
            (Some(backup_file), Some(previous_file), Some(current_file)) => {
                let local_changed = previous_file != current_file;
                let backup_changed = previous_file != backup_file;
                match (local_changed, backup_changed) {
                    (false, false) => self.unchanged_file(current_file),
                    (true, false) => self.push_to_backup(current_file),
                    (false, true) => self.pull_from_backup(backup_file),
                    (true, true) => panic!("{}: Has been modified both locally and in backup, for now we crash :(", self.file_path),
                }
            },

            (None, _, Some(current_file)) => self.push_to_backup(current_file),
            (Some(backup_file), _, None) => self.pull_from_backup(backup_file),

            (None, Some(_), None) => self.removed_file(),
            (Some(_), None, Some(_)) => panic!("{}: Exists locally but was not pulled down correctly", self.file_path),
            (None, None, None) => panic!("{}: Attempting to sync a file not being tracked anywhere", self.file_path),
        }
    }

    fn unchanged_file(&self, current_file: &FileState) -> Option<FileState> {
        println!("{}: Has not been modified, nothing to do", self.file_path);
        Some(current_file.clone())
    }

    fn removed_file(&self) -> Option<FileState> {
        println!("{}: Was removed locally and from backup, nothing to do", self.file_path);
        None
    }

    fn push_to_backup(&self, current_file: &FileState) -> Option<FileState> {
        println!("{}: Has been modified locally, pushing to backup", self.file_path);
        self.backup_config.push(current_file);
        Some(current_file.clone())
    }

    fn pull_from_backup(&self, backup_file: &FileState) -> Option<FileState> {
        println!("{}: Has been modified in backup, pulling from backup", self.file_path);
        self.backup_config.pull(backup_file);
        Some(backup_file.clone())
    }
}
