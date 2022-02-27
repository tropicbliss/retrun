use crate::cli::Guess;
enum Rule {
    NotContains(char),
    Contains(char, usize),
    Correct(char, usize),
}

pub fn filter_words(words: Vec<String>, guess_units: Vec<Guess>) -> Vec<String> {
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
        })
        .collect();
    let correct_rules: Vec<_> = rules
        .iter()
        .filter_map(|rule| {
            if let Rule::Correct(letter, idx) = rule {
                return Some((*letter, *idx));
            }
            None
        })
        .collect();
    words
        .into_iter()
        .filter(|word| {
            rules.iter().all(|rule| match rule {
                Rule::NotContains(letter) => {
                    !word.contains(*letter) || {
                        let correct_idxs: Vec<_> = correct_rules
                            .iter()
                            .filter(|data| data.0 == *letter)
                            .map(|data| data.1)
                            .collect();
                        word.chars()
                            .enumerate()
                            .all(|(idx, l)| l != *letter || correct_idxs.contains(&idx))
                    }
                }
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
