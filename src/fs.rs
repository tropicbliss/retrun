use std::{fs::read_to_string, path::PathBuf};

pub fn get_words(path: PathBuf) -> Vec<String> {
    let file = read_to_string(&path).expect(&format!("Unable to open {}", path.display()));
    file.lines().map(ToString::to_string).collect()
}
