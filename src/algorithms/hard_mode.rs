use crate::{dictionary, Guesser};

pub struct HardMode;

impl Guesser for HardMode {
    fn guess(&self, mut words: Vec<&'static str>) -> &'static str {
        words.sort_unstable_by_key(|word| dictionary::WORDS[word]);
        words.last().expect("Unable to find any words")
    }
}

impl HardMode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for HardMode {
    fn default() -> Self {
        Self::new()
    }
}
