# retrun

## About

This is a fast Wordle solver based on the algorithm 3blue1brown uses.

## Usage example

Output the best word and the number of eligible words in easy mode, while blocking the words "light" and "night".

```sh
retrun -e -b light -b night -c -- -----:00000,tares:21111,teach:21112,tweak:21111,might:13333
```

The syntax of `state` is based on the syntax [Wordle botfights](https://botfights.ai/game/wordle) uses. Run `retrun --help` for more usage info.

## Building

```sh
git clone https://github.com/tropicbliss/retrun
cd retrun
cargo build --release
```

Compiling from source requires the latest stable version of Rust. Older Rust versions may be able to compile `retrun`, but they are not guaranteed to keep working.

The binary will be located in `target/release`.

Note: This project uses a build script (`build.rs` in this project's root) to generate a static HashMap at compile time.
