use anyhow::Result;
use aoc_getter::aoc::get_aoc_puzzle_input;
use clap::Parser;

/// CLI arguments for Advent of Code puzzle input fetcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of Advent of Code puzzle to fetch (1-25)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Fetch input for Day 1
    match get_aoc_puzzle_input(args.day) {
        Ok(input) => {
            println!("Puzzle input for day {}:\n{}", 1, input);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error fetching puzzle input: {}", e);
            Err(e)
        }
    }
}
