#![warn(clippy::pedantic)]

use clap::Parser;
use std::io::{stdout, Result, Write};

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
/// Example:
///
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
    let history = retrun::get_guesses(&args.state);
    let filtered_data = retrun::algorithm::guess(&history);
    render(filtered_data.0, args.count.then(|| filtered_data.1)).expect("Failed to render CLI");
}

fn render(word: &str, word_count: Option<usize>) -> Result<()> {
    if let Some(word_count) = word_count {
        writeln!(stdout(), "{} ({})", word, word_count)?;
    } else {
        writeln!(stdout(), "{}", word)?;
    }
    Ok(())
}
