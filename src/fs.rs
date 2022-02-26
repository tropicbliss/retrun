use std::{fs::read_to_string, path::PathBuf};

pub fn get_words(path: PathBuf) -> Vec<String> {
    let file = read_to_string(path).unwrap();
    file.lines().map(ToString::to_string).collect()
}
