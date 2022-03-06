use crate::{dictionary, Guesser, Rule};
use std::collections::HashMap;

pub struct NormalMode;

pub struct Candidate {
    word: &'static str,
    goodness: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Grey
    Wrong,
}

impl Correctness {
    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

impl Guesser for NormalMode {
    fn guess(&self, mut words: Vec<&'static str>) -> &'static str {
        if words.len() == 1 {
            return words[0];
        }
        words.sort_unstable_by_key(|word| std::cmp::Reverse(dictionary::WORDS[word]));
        let mut patterns: Vec<_> = Correctness::patterns().collect();
        let remaining_count: usize = words.iter().map(|word| dictionary::WORDS[word]).sum();
        let mut best: Option<Candidate> = None;
        let mut i = 0;
        let stop = (words.len() / 3).max(20);
        for word in &words {
            let count = dictionary::WORDS[word];
            let mut sum = 0.0;
            let check_pattern = |pattern: &[Correctness; 5]| {
                let mut in_pattern_total = 0;
                for candidate in &words {
                    let g: Vec<_> = word
                        .bytes()
                        .zip(pattern.iter())
                        .enumerate()
                        .map(|(idx, (letter, rule))| match rule {
                            Correctness::Correct => Rule::Correct(letter, idx),
                            Correctness::Misplaced => Rule::Misplaced(letter, idx),
                            Correctness::Wrong => Rule::Wrong(letter),
                        })
                        .collect();
                    if matches(candidate, g) {
                        in_pattern_total += dictionary::WORDS[candidate];
                    }
                }
                if in_pattern_total == 0 {
                    return false;
                }
                // TODO: apply sigmoid
                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += p_of_this_pattern * p_of_this_pattern.log2();
                true
            };
            patterns.retain(check_pattern);
            let p_word = count as f64 / remaining_count as f64;
            let goodness = p_word * -sum;
            if let Some(c) = &best {
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness });
                }
            } else {
                best = Some(Candidate { word, goodness });
            }
            i += 1;
            if i >= stop {
                break;
            }
        }
        best.expect("Unable to find any words").word
    }
}

impl NormalMode {
    pub fn new() -> Self {
        Self {}
    }
}

fn matches(word: &'static str, ruleset: Vec<Rule>) -> bool {
    let mut possible_lengths: HashMap<u8, usize> = HashMap::new();
    ruleset
        .iter()
        .filter_map(|rule| match rule {
            Rule::Wrong(letter) => Some(letter),
            _ => None,
        })
        .for_each(|letter| {
            let number_of_occurences = ruleset
                .iter()
                .filter_map(|rule| match rule {
                    Rule::Misplaced(l, _) | Rule::Correct(l, _) if l == letter => Some(letter),
                    _ => None,
                })
                .count();
            possible_lengths.insert(*letter, number_of_occurences);
        });
    ruleset.iter().all(|rule| match rule {
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
                && !word.bytes().any(|l| letter == &l)
        }
    })
}

impl Default for NormalMode {
    fn default() -> Self {
        Self::new()
    }
}
