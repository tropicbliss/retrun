#![warn(clippy::pedantic)]

use clap::Parser;
use std::io::{self, Result, Write};

#[derive(Parser)]
#[clap(author, version, about)]
/// Feedback character legend:\n
/// Initial: 0\n
/// Gray: 1\n
/// Yellow: 2\n
/// Green: 3\n
///
/// Example:\n
/// -----:00000,arose:31112,amend:31211
pub struct Args {
    /// Play state
    #[clap(global = true)]
    pub state: String,

    /// Show number of results
    #[clap(short, long)]
    pub count: bool,
}

fn main() {
    let args = Args::parse();
    let guess_units = retrun::get_guesses(&args.state);
    let filtered_data = retrun::filter_words(guess_units);
    let best_word = retrun::algorithm::guess(filtered_data.1);
    render(best_word, filtered_data.0, args.count).expect("Failed to render CLI");
}

fn render(word: &str, word_count: usize, count: bool) -> Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut handle = io::BufWriter::new(handle);
    write!(handle, "{}", word)?;
    if count {
        write!(handle, " ({})", word_count)?;
    }
    writeln!(handle)?;
    Ok(())
}
