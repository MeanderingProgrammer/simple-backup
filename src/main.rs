use glob::glob;
use std::path::PathBuf;

fn main() {
    get_directory_state("C:/Users/vsusl/Documents/other/important");
    get_directory_state("target/release");
}

fn get_directory_state(root: &str) {
    let glob_pattern = format!("{}/**/*", root);

    let file_paths: Vec<PathBuf> = glob(&glob_pattern).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .collect();

    for file_path in file_paths {
        println!("{:?}", file_path);
    }
}
