use crate::{dictionary, enumerate_mask, Correctness, Guess, MAX_MASK_ENUM};

pub struct Candidate {
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
    (entropy * 3.264 + 4.834).ln()
}

pub fn guess(history: &[Guess]) -> (&'static str, usize) {
    if history.is_empty() {
        return ("tares", dictionary::WORDS.len());
    }
    let score = history.len() as f64 + 1.0;
    let sum: f64 = dictionary::WORDS
        .into_iter()
        .map(|entry| *entry.1 as f64)
        .sum();
    let mut remaining: Vec<_> = dictionary::WORDS
        .into_iter()
        .map(|entry| entry.0)
        .filter(|word| history.iter().all(|guess| guess.matches(word)))
        .map(|word| {
            (
                word,
                sigmoid(*dictionary::WORDS.get(word).unwrap() as f64 / sum),
            )
        })
        .collect();
    remaining.sort_unstable_by_key(|(word, _)| std::cmp::Reverse(dictionary::WORDS.get(word)));
    let remaining_p: f64 = remaining.iter().map(|entry| entry.1).sum();
    let remaining_entropy = -remaining
        .iter()
        .map(|(_, p)| {
            let p = p / remaining_p;
            p * p.log2()
        })
        .sum::<f64>();
    let mut best: Option<Candidate> = None;
    let mut i = 0;
    let stop = (remaining.len() / 3).max(20);
    for (word, count) in &remaining {
        let mut totals = [0.0f64; MAX_MASK_ENUM];
        for (candidate, count) in &remaining {
            let idx = enumerate_mask(&Correctness::compute(candidate, word));
            totals[idx] += count;
        }
        let sum: f64 = totals
            .into_iter()
            .filter(|t| *t != 0.0)
            .map(|t| {
                let p_of_this_pattern = t / remaining_p;
                p_of_this_pattern * p_of_this_pattern.log2()
            })
            .sum();
        let p_word = *count / remaining_p;
        let e_info = -sum;
        let e_score =
            p_word * score + (1.0 - p_word) * (score + est_steps_left(remaining_entropy - e_info));
        if let Some(c) = &best {
            if e_score > c.e_score {
                best = Some(Candidate { word, e_score });
            }
        } else {
            best = Some(Candidate { word, e_score });
        }
        i += 1;
        if i >= stop {
            break;
        }
    }
    (
        best.expect("Unable to find any words").word,
        remaining.len(),
    )
}
