use std::env;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const DICTIONARY_FILE: &str = "dictionary.txt";

fn main() {
    let file =
        read_to_string(DICTIONARY_FILE).expect(&format!("Unable to open {}", DICTIONARY_FILE));
    let word_data: Vec<_> = file
        .lines()
        .map(|line| {
            let (word, count) = line
                .split_once(' ')
                .expect("every line is word + space + frequency");
            if count.parse::<usize>().is_err() {
                panic!("every count is a number");
            }
            (word, count)
        })
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
    write!(&mut file, ";\n").unwrap();
}
