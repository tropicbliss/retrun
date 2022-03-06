#![warn(clippy::pedantic)]

use clap::Parser;
use std::path::{Path, PathBuf};

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

fn main() {
    let args = Args::parse();
    play(&args.state, &args.wordlist, args.count);
}

fn play(state: &str, wordlist: &Path, count: bool) {
    let guess_units = wsol::get_guesses(state);
    let words = wsol::get_words(wordlist);
    let char_scores = wsol::CharScore::new(&words);
    let filtered_words = wsol::filter_words(words, guess_units);
    wsol::render(filtered_words, &char_scores, count).expect("Failed to render CLI");
}
