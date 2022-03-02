use clap::Parser;
use std::io::{self, Result, Write};
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser)]
#[clap(author, version, about)]
/// Feedback character legend:
///
/// Initial: 0
///
/// Gray: 1
///
/// Yellow: 2
///
/// Green: 3
///
///
/// Example:
///
/// -----:00000,arose:31112,amend:31211
pub struct Args {
    /// Play state
    #[clap(global = true)]
    pub state: String,

    /// Input file
    #[clap(short, long, default_value = "wordlist.txt")]
    pub wordlist: PathBuf,

    /// Show number of results
    #[clap(short, long)]
    pub count: bool,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}

pub struct Guess {
    pub guess: String,
    pub feedback: String,
}

pub fn get_guesses(state: &str) -> Vec<Guess> {
    let guesses = state.split(',');
    guesses
        .into_iter()
        .map(|guess| {
            let mut guess_data = guess.split(':');
            Guess {
                guess: guess_data
                    .next()
                    .expect("Guess segment not found")
                    .to_string(),
                feedback: guess_data
                    .next()
                    .expect("Feedback segment not found")
                    .to_string(),
            }
        })
        .collect()
}

pub struct CharScore {
    char_score: HashMap<char, usize>,
}

impl CharScore {
    pub fn new(words: &[String]) -> Self {
        let mut char_score = HashMap::new();
        for word in words {
            for ch in word.chars() {
                let counter = char_score.entry(ch).or_insert(0);
                *counter += 1;
            }
        }
        Self { char_score }
    }

    fn get_word_score(&self, word: &str) -> usize {
        let mut letters = word.chars().collect();
        true_dedup(&mut letters);
        letters
            .into_iter()
            .map(|letter| self.char_score.get(&letter).unwrap())
            .sum()
    }
}

fn true_dedup<T>(vec: &mut Vec<T>)
where
    T: Ord,
{
    vec.sort_unstable();
    vec.dedup();
}

pub fn render(mut words: Vec<String>, score_info: &CharScore, show_count: bool) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    words.sort_by_cached_key(|word| score_info.get_word_score(word));
    write!(
        handle,
        "{}",
        words.last().expect("Unable to find any words")
    )?;
    if show_count {
        write!(handle, " ({})", words.len())?;
    }
    Ok(())
}
