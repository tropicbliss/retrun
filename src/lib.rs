pub mod algorithm;
pub mod dictionary;

use std::collections::HashMap;

pub struct Guess {
    pub word: String,
    pub mask: String,
}

pub fn get_guesses(state: &str) -> Vec<Guess> {
    if !state.is_ascii() {
        panic!("Invalid characters in state");
    }
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

enum Rule {
    /// Grey
    Wrong(u8),
    /// Yellow
    Misplaced(u8, usize),
    /// Green
    Correct(u8, usize),
}

pub fn filter_words(history: Vec<Guess>) -> (usize, Vec<&'static str>) {
    let mut possible_lengths: HashMap<u8, usize> = HashMap::new();
    let rules: Vec<_> = history
        .into_iter()
        .flat_map(|unit| {
            let result: Vec<_> = unit
                .word
                .bytes()
                .zip(unit.mask.bytes())
                .enumerate()
                .filter_map(|(idx, data)| match data.1 {
                    b'1' => Some(Rule::Wrong(data.0)),
                    b'2' => Some(Rule::Misplaced(data.0, idx)),
                    b'3' => Some(Rule::Correct(data.0, idx)),
                    b'0' => None,
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
    if rules.is_empty() {
        return (dictionary::WORDS.len(), vec!["tares"]);
    }
    let filtered_words: Vec<_> = dictionary::WORDS
        .into_iter()
        .map(|entry| entry.0)
        .filter(|word| {
            rules.iter().all(|rule| match rule {
                Rule::Wrong(letter) => {
                    !word.bytes().any(|l| letter == &l)
                        || &word.bytes().filter(|l| l == letter).count()
                            == possible_lengths.get(letter).unwrap()
                }
                Rule::Correct(letter, idx) => {
                    word.as_bytes().get(*idx).expect("Unexpected word length") == letter
                }
                Rule::Misplaced(letter, idx) => {
                    word.as_bytes().get(*idx).expect("Unexpected word length") != letter
                        && word.bytes().any(|l| letter == &l)
                }
            })
        })
        .copied()
        .collect();
    (filtered_words.len(), filtered_words)
}
