use std::rc::Rc;

use crate::{enumerate_mask, Correctness, Guess, MAX_MASK_ENUM};

include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));

struct Candidate {
    word: &'static str,
    e_score: f64,
}

const L: f64 = 1.0;
const K: f64 = 30000000.0;
const X0: f64 = 0.00000497;

fn sigmoid(p: f64) -> f64 {
    L / (1.0 + (-K * (p - X0)).exp())
}

fn est_steps_left(entropy: f64) -> f64 {
    (entropy * 3.870 + 3.679).ln()
}

pub struct Algorithm {
    pub guess: &'static str,
    pub count: usize,
}

impl Algorithm {
    #[must_use]
    pub fn guess(history: &[Guess], blocked: &[String], easy_mode: bool) -> Self {
        if history.is_empty() && !blocked.contains(&"tares".to_string()) {
            return Self {
                guess: "tares",
                count: WORDS.len(),
            };
        }
        let sum: f64 = WORDS.into_iter().map(|(_, count)| *count as f64).sum();
        let (consider, remaining) = if easy_mode {
            let consider: Vec<_> = WORDS
                .into_iter()
                .map(|(word, _)| word)
                .filter(|word| !blocked.contains(&(**word).to_string()))
                .map(|word| (word, sigmoid(*WORDS.get(word).unwrap() as f64 / sum)))
                .collect();
            let remaining: Vec<_> = consider
                .iter()
                .filter(|(word, _)| history.iter().all(|guess| guess.matches(word)))
                .copied()
                .collect();
            (Rc::new(consider), Rc::new(remaining))
        } else {
            let consider: Rc<Vec<_>> = Rc::new(
                WORDS
                    .into_iter()
                    .map(|(word, _)| word)
                    .filter(|word| {
                        history.iter().all(|guess| {
                            guess.matches(word) && !blocked.contains(&(**word).to_string())
                        })
                    })
                    .map(|word| (word, sigmoid(*WORDS.get(word).unwrap() as f64 / sum)))
                    .collect(),
            );
            let remaining = Rc::clone(&consider);
            (consider, remaining)
        };
        let remaining_len = remaining.len();
        if remaining_len == 1 {
            return Self {
                guess: remaining.first().unwrap().0,
                count: remaining_len,
            };
        }
        let score = history.len() as f64;
        let remaining_p: f64 = remaining.iter().map(|(_, p)| p).sum();
        let remaining_entropy = -remaining
            .iter()
            .map(|(_, p)| {
                let p = p / remaining_p;
                p * p.log2()
            })
            .sum::<f64>();
        let mut best: Option<Candidate> = None;
        let mut i = 0;
        let stop = (remaining.len() / 3).max(20).min(remaining_len);
        for (word, count) in consider.iter() {
            let mut totals = [0.0f64; MAX_MASK_ENUM];
            let mut in_remaining = false;
            for (candidate, count) in remaining.iter() {
                in_remaining |= word == candidate;
                let idx = enumerate_mask(&Correctness::compute(candidate, word));
                totals[idx] += count;
            }
            let sum: f64 = totals
                .into_iter()
                .filter(|t| *t != 0.0)
                .map(|p| {
                    let p_of_this_pattern = p / remaining_p;
                    p_of_this_pattern * p_of_this_pattern.log2()
                })
                .sum();
            let p_word = if in_remaining {
                count / remaining_p
            } else {
                0.0
            };
            let e_info = -sum;
            let e_score = p_word * (score + 1.0)
                + (1.0 - p_word) * (score + est_steps_left(remaining_entropy - e_info));
            if let Some(c) = &best {
                if e_score < c.e_score {
                    best = Some(Candidate { word, e_score });
                }
            } else {
                best = Some(Candidate { word, e_score });
            }
            if in_remaining {
                i += 1;
                if i >= stop {
                    break;
                }
            }
        }
        Self {
            guess: best.expect("Unable to find any words").word,
            count: remaining_len,
        }
    }
}
