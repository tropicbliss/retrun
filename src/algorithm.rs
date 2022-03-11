use crate::{dictionary, enumerate_mask, Correctness, Guess, MAX_MASK_ENUM};

pub struct Candidate {
    word: &'static str,
    goodness: f64,
}

pub fn guess(history: &[Guess]) -> (&'static str, usize) {
    if history.is_empty() {
        return ("tares", dictionary::WORDS.len());
    }
    let mut remaining: Vec<_> = dictionary::WORDS
        .into_iter()
        .map(|entry| entry.0)
        .filter(|word| history.iter().all(|guess| guess.matches(word)))
        .collect();
    remaining.sort_unstable_by_key(|word| std::cmp::Reverse(dictionary::WORDS.get(word)));
    let remaining_count: usize = remaining
        .iter()
        .map(|word| dictionary::WORDS.get(word).unwrap())
        .sum();
    let mut best: Option<Candidate> = None;
    let mut i = 0;
    let stop = (remaining.len() / 3).max(20);
    for word in &remaining {
        let count = dictionary::WORDS.get(word).unwrap();
        let mut totals = [0usize; MAX_MASK_ENUM];
        for candidate in &remaining {
            let idx = enumerate_mask(&Correctness::compute(candidate, word));
            totals[idx] += count;
        }
        let sum: f64 = totals
            .into_iter()
            .filter(|t| *t != 0)
            .map(|t| {
                let p_of_this_pattern = t as f64 / remaining_count as f64;
                p_of_this_pattern * p_of_this_pattern.log2()
            })
            .sum();
        let p_word = *count as f64 / remaining_count as f64;
        let entropy = -sum;
        let goodness = p_word * entropy;
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
    (
        best.expect("Unable to find any words").word,
        remaining.len(),
    )
}
