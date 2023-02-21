use std::fs::{create_dir_all, OpenOptions};
use std::path::Path;

pub fn create_file_at_path(path: &Path) {
    // Create the file only if it does not already exist
    if !path.exists() {
        // Create the directory structure if needed
        let directory = path.parent().unwrap();
        if !directory.exists() {
            create_dir_all(directory).unwrap();
        }
        // Now we create the file in the specified path
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
    }
}
