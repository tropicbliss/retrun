use std::{fs::read_to_string, path::Path};

pub fn get_words(path: &Path) -> Vec<String> {
    let file =
        read_to_string(&path).unwrap_or_else(|_| panic!("Unable to open {}", path.display()));
    file.lines().map(ToString::to_string).collect()
}
