#![warn(clippy::pedantic)]

mod cli;
mod finder;
mod fs;

fn main() {
    let args = cli::Args::new();
    let guess_units = cli::get_guesses(&args.state);
    let words = fs::get_words(&args.wordlist);
    let char_scores = cli::CharScore::new(&words);
    let filtered_words = finder::filter_words(words, guess_units);
    cli::render(filtered_words, &char_scores);
}
