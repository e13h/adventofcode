use aoc_getter::aoc::get_aoc_puzzle_input;
use std::num::ParseIntError;
use std::result::Result;

fn parse_input(puzzle_input: String) -> Result<Vec<Vec<i32>>, ParseIntError> {
    puzzle_input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|value| value.trim().parse::<i32>())
                .collect()
        })
        .collect()
}

fn main() {
    let input = get_aoc_puzzle_input(2).unwrap();
    let reports = parse_input(input).unwrap();
    println!("Found {} reports", reports.len());
}
