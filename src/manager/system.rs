use crate::backup::backup::BackupConfig;
use crate::db::profile::DirectoryConfig;
use crate::db::state::SystemState;
use crate::manager::file::FileStateManager;

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
         FileStateManager::new(
            self.backup_config,
            file_path,
            self.backup_state.get(file_path),
            self.previous_state.get(file_path),
            self.current_state.get(file_path),
        )
     }
 }
