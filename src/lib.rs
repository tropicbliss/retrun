use std::collections::HashMap;
use std::io::{self, Result, Write};
use std::{fs::read_to_string, path::Path};

pub struct Guess {
    pub word: String,
    pub mask: String,
}

pub fn get_guesses(state: &str) -> Vec<Guess> {
    let guesses = state.split(',');
    guesses
        .into_iter()
        .map(|guess| {
            let mut guess_data = guess.split(':');
            Guess {
                word: guess_data
                    .next()
                    .expect("Word segment not found")
                    .to_string(),
                mask: guess_data
                    .next()
                    .expect("Mask segment not found")
                    .to_string(),
            }
        })
        .collect()
}

pub struct CharScore {
    char_score: HashMap<char, usize>,
}

impl CharScore {
    pub fn new(words: &[String]) -> Self {
        let mut char_score = HashMap::new();
        for word in words {
            for ch in word.chars() {
                let counter = char_score.entry(ch).or_insert(0);
                *counter += 1;
            }
        }
        Self { char_score }
    }

    fn get_word_score(&self, word: &str) -> usize {
        let mut letters = word.chars().collect();
        true_dedup(&mut letters);
        letters
            .into_iter()
            .map(|letter| self.char_score.get(&letter).unwrap())
            .sum()
    }
}

fn true_dedup<T>(vec: &mut Vec<T>)
where
    T: Ord,
{
    vec.sort_unstable();
    vec.dedup();
}

pub fn render(mut words: Vec<String>, score_info: &CharScore, show_count: bool) -> Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut handle = io::BufWriter::new(handle);
    words.sort_by_cached_key(|word| score_info.get_word_score(word));
    write!(
        handle,
        "{}",
        words.last().expect("Unable to find any words")
    )?;
    if show_count {
        write!(handle, " ({})", words.len())?;
    }
    writeln!(handle)?;
    Ok(())
}

enum Rule {
    Wrong(char),
    Misplaced(char, usize),
    Correct(char, usize),
}

pub fn filter_words(words: Vec<String>, guesses: Vec<Guess>) -> Vec<String> {
    let mut possible_lengths: HashMap<char, usize> = HashMap::new();
    let rules: Vec<_> = guesses
        .into_iter()
        .flat_map(|unit| {
            let result: Vec<_> = unit
                .word
                .chars()
                .zip(unit.mask.chars())
                .enumerate()
                .filter_map(|(idx, data)| match data.1 {
                    '1' => Some(Rule::Wrong(data.0)),
                    '2' => Some(Rule::Misplaced(data.0, idx)),
                    '3' => Some(Rule::Correct(data.0, idx)),
                    '0' => None,
                    _ => unimplemented!("Unexpected feedback segment character"),
                })
                .collect();
            result
                .iter()
                .filter_map(|rule| match rule {
                    Rule::Wrong(letter) => Some(letter),
                    _ => None,
                })
                .for_each(|letter| {
                    let number_of_occurences = result
                        .iter()
                        .filter_map(|rule| match rule {
                            Rule::Misplaced(l, _) | Rule::Correct(l, _) if l == letter => {
                                Some(letter)
                            }
                            _ => None,
                        })
                        .count();
                    possible_lengths.insert(*letter, number_of_occurences);
                });
            result
        })
        .collect();
    words
        .into_iter()
        .filter(|word| {
            rules.iter().all(|rule| match rule {
                Rule::Wrong(letter) => {
                    !word.contains(*letter)
                        || &word.chars().filter(|l| l == letter).count()
                            == possible_lengths.get(letter).unwrap()
                }
                Rule::Correct(letter, idx) => {
                    &word.chars().nth(*idx).expect("Unexpected word length") == letter
                }
                Rule::Misplaced(letter, idx) => {
                    &word.chars().nth(*idx).expect("Unexpected word length") != letter
                        && word.contains(*letter)
                }
            })
        })
        .collect()
}

pub fn get_words(path: &Path) -> Vec<String> {
    let file =
        read_to_string(&path).unwrap_or_else(|_| panic!("Unable to open {}", path.display()));
    file.split_whitespace().map(ToString::to_string).collect()
}
