use bincode;
use serde::{
    Serialize,
    de::DeserializeOwned,
};
use std::fs::{
    create_dir_all,
    File,
    OpenOptions,
};
use std::io::prelude::*;
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

pub fn read<T: DeserializeOwned>(root: &str, file_name: &str, default: T) -> T {
    let path = Path::new(root).join(file_name);
    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = Vec::<u8>::new();
            file.read_to_end(&mut buffer).unwrap();
            bincode::deserialize(&buffer).unwrap()
        },
        Err(_) => default,
    }
}

pub fn save<T: Serialize>(root: &str, file_name: &str, data: &T) {
    let encoded = bincode::serialize(data).unwrap();
    let path = Path::new(root).join(file_name);
    let mut file = File::create(path).unwrap();
    file.write_all(&encoded).unwrap();
}
