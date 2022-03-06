#![warn(clippy::pedantic)]

use clap::Parser;
use retrun::Guesser;
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

    /// Hard mode
    #[clap(short, long)]
    pub hard_mode: bool,
}

fn main() {
    let args = Args::parse();
    if args.hard_mode {
        play(retrun::algorithms::HardMode::new, args)
    } else {
        play(retrun::algorithms::NormalMode::new, args)
    }
    .expect("Failed to render CLI");
}

fn play<G>(mut mk: impl FnMut() -> G, args: Args) -> Result<()>
where
    G: Guesser,
{
    let guess_units = retrun::get_guesses(&args.state);
    let filtered_words = retrun::filter_words(guess_units);
    let guesser = (mk)();
    let word_count = filtered_words.len();
    let best_word = retrun::Wordle::play(filtered_words, guesser);
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut handle = io::BufWriter::new(handle);
    write!(handle, "{}", best_word)?;
    if args.count {
        write!(handle, " ({})", word_count)?;
    }
    writeln!(handle)?;
    Ok(())
}
