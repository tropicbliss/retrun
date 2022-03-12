use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let file = read_to_string("dictionary.txt").unwrap();
    let word_data: Vec<_> = file
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect();
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("dictionary.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let mut builder = phf_codegen::Map::new();
    for (word, count) in word_data {
        builder.entry(word, count);
    }
    write!(
        &mut file,
        "static WORDS: phf::Map<&'static str, usize> = {}",
        builder.build()
    )
    .unwrap();
    writeln!(&mut file, ";").unwrap();
}
