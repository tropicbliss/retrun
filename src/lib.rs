pub mod algorithms;
pub mod dictionary;

use std::collections::HashMap;

pub struct Guess {
    pub word: String,
    pub mask: String,
}

pub trait Guesser {
    fn guess(&self, words: Vec<&'static str>) -> &'static str;
}

pub struct Wordle;

impl Wordle {
    pub fn play<G: Guesser>(filtered_words: Vec<&'static str>, guesser: G) -> &'static str {
        guesser.guess(filtered_words)
    }
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

enum Rule {
    /// Grey
    Wrong(char),
    /// Yellow
    Misplaced(char, usize),
    /// Green
    Correct(char, usize),
}

pub fn filter_words(history: Vec<Guess>) -> Vec<&'static str> {
    let mut possible_lengths: HashMap<char, usize> = HashMap::new();
    let rules: Vec<_> = history
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
    dictionary::WORDS
        .into_iter()
        .map(|entry| entry.0)
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
        .copied()
        .collect()
}
