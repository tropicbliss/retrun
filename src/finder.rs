use crate::cli::Guess;

enum Rule {
    NotContains(char),
    Contains(char, usize),
    Correct(char, usize),
}

pub fn filter_words(words: Vec<String>, guess_units: Vec<Guess>) -> Vec<String> {
    let rules: Vec<_> = guess_units
        .into_iter()
        .map(|unit| {
            let result: Vec<_> = unit
                .guess
                .chars()
                .zip(unit.feedback.chars())
                .enumerate()
                .map(|(idx, data)| match data.1 {
                    '1' => Some(Rule::NotContains(data.0)),
                    '2' => Some(Rule::Contains(data.0, idx)),
                    '3' => Some(Rule::Correct(data.0, idx)),
                    '0' => None,
                    _ => unimplemented!("Unexpected feedback segment character"),
                })
                .flatten()
                .collect();
            result
        })
        .flatten()
        .collect();
    words
        .into_iter()
        .filter(|word| {
            rules.iter().all(|rule| match rule {
                Rule::NotContains(letter) => !word.contains(*letter),
                Rule::Correct(letter, idx) => {
                    word.chars().nth(*idx).expect("Unexpected word length") == *letter
                }
                Rule::Contains(letter, idx) => {
                    word.chars().nth(*idx).expect("Unexpected word length") != *letter
                }
            })
        })
        .collect()
}
