use crate::cli::Guess;
use std::collections::HashMap;

enum Rule {
    NotContains(char),
    Contains(char, usize),
    Correct(char, usize),
}

pub fn filter_words(words: Vec<String>, guess_units: Vec<Guess>) -> Vec<String> {
    let mut possible_lengths: HashMap<char, usize> = HashMap::new();
    let rules: Vec<_> = guess_units
        .into_iter()
        .flat_map(|unit| {
            let result: Vec<_> = unit
                .guess
                .chars()
                .zip(unit.feedback.chars())
                .enumerate()
                .filter_map(|(idx, data)| match data.1 {
                    '1' => Some(Rule::NotContains(data.0)),
                    '2' => Some(Rule::Contains(data.0, idx)),
                    '3' => Some(Rule::Correct(data.0, idx)),
                    '0' => None,
                    _ => unimplemented!("Unexpected feedback segment character"),
                })
                .collect();
            result
                .iter()
                .filter_map(|rule| match rule {
                    Rule::NotContains(letter) => Some(letter),
                    _ => None,
                })
                .for_each(|letter| {
                    let number_of_occurences = result
                        .iter()
                        .filter_map(|rule| match rule {
                            Rule::Contains(l, _) | Rule::Correct(l, _) if l == letter => {
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
                Rule::NotContains(letter) => {
                    !word.contains(*letter)
                        || &word.chars().filter(|l| l == letter).count()
                            == possible_lengths.get(letter).unwrap()
                }
                Rule::Correct(letter, idx) => {
                    &word.chars().nth(*idx).expect("Unexpected word length") == letter
                }
                Rule::Contains(letter, idx) => {
                    &word.chars().nth(*idx).expect("Unexpected word length") != letter
                        && word.contains(*letter)
                }
            })
        })
        .collect()
}
