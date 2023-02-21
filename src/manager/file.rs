use crate::backup::backup::BackupConfig;
use crate::db::state::FileState;

#[derive(Debug)]
pub struct FileStateManager<'a> {
    backup_config: &'a BackupConfig,
    file_path: &'a str,
    backup_file: Option<&'a FileState>,
    previous_file: Option<&'a FileState>,
    current_file: Option<&'a FileState>,
}

impl<'a> FileStateManager<'a> {
    pub fn new(
        backup_config: &'a BackupConfig,
        file_path: &'a str,
        backup_file: Option<&'a FileState>,
        previous_file: Option<&'a FileState>,
        current_file: Option<&'a FileState>,
    ) -> Self {
        Self {
            backup_config,
            file_path,
            backup_file,
            previous_file,
            current_file,
        }
    }

    pub fn sync(&self) -> Option<FileState> {
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
