use crate::cli::GuessUnit;

enum Guess {
    NotContains(char),
    Contains(char, usize),
    Correct(char, usize),
}

pub fn filter_words(words: Vec<String>, guess_units: Vec<GuessUnit>) -> Vec<String> {
    let rules: Vec<_> = guess_units
        .into_iter()
        .map(|unit| {
            let result: Vec<Guess> = unit
                .guess
                .chars()
                .zip(unit.feedback.chars())
                .enumerate()
                .map(|(idx, data)| match data.1 {
                    '1' => Some(Guess::NotContains(data.0)),
                    '2' => Some(Guess::Contains(data.0, idx)),
                    '3' => Some(Guess::Correct(data.0, idx)),
                    '0' => None,
                    _ => unimplemented!(),
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
                Guess::NotContains(ch) => !word.contains(*ch),
                Guess::Correct(ch, idx) => word.chars().nth(*idx).unwrap() == *ch,
                Guess::Contains(ch, idx) => word.chars().nth(*idx).unwrap() != *ch,
            })
        })
        .collect()
}
