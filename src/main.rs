mod cli;
mod finder;
mod fs;

fn main() {
    let args = cli::Args::new();
    let guess_units = cli::get_guess_unit(args.state);
    let words = fs::get_words(args.wordlist);
    let char_score = cli::CharScore::new(&words);
    let filtered_words = finder::filter_words(words, guess_units);
    cli::render(filtered_words, char_score);
}
