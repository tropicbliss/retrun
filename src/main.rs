#![warn(clippy::pedantic)]

use clap::Parser;
use std::io::{self, Result, Write};

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
    render(filtered_data.0, filtered_data.1, args.count).expect("Failed to render CLI");
}

fn render(word: &str, word_count: usize, count: bool) -> Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut handle = io::BufWriter::with_capacity(13, handle);
    write!(handle, "{}", word)?;
    if count {
        write!(handle, " ({})", word_count)?;
    }
    writeln!(handle)?;
    handle.flush()?;
    Ok(())
}
