# pangran

<img align="right" src="https://raw.githubusercontent.com/BimoT/pangran/assets/logo.svg" alt="pangran logo">

Pangran is a simple TUI program that checks if you've typed a [pangram](https://en.wikipedia.org/wiki/Pangram). The most famous pangram is undoubtedly "The quick brown fox jumps over the lazy dog". This program was created using the Rust TUI library [ratatui](https://github.com/tui-rs-revival/ratatui) and terminal manipulation library [crossterm](https://github.com/crossterm-rs/crossterm). Although this program was created and tested on Linux, these two libraries should allow it to work on Windows as well.

## Usage

Type `pangram -h` to show help, and `pangram -v` to show the version.
Just type `pangram` in your terminal and you'll be launched into the TUI. Pressing the Escape key quits the TUI.

![Example]("https://raw.githubusercontent.com/BimoT/pangran/assets/example.gif")

## Installing

The easiest way to install pangran is to download the code and build it with cargo.

```
git clone https://github.com/BimoT/pangran
cd pangran
cargo build --release
```
This requires you to have cargo, and the rust toolchain installed.

## Todo

- [] Implement cursor behaviour with line wrapping. Currently, the ratatui line wrapping happens at word boundaries, which makes implementing cursor behaviour very difficult.
- [] Add animation when a pangram is detected?
