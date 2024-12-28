use aoc_getter::aoc::get_aoc_puzzle_input;
use std::result::Result;

fn parse_input(puzzle_input: String) -> Result<Vec<Vec<i32>>, String> {
    let mut result = Vec::new();
    for line in puzzle_input.lines() {
        let mut report = Vec::new();
        result.push(report);
    }
    Ok(result)
}

fn main() {
    let input = get_aoc_puzzle_input(2).unwrap();
    let reports = parse_input(input).unwrap();
    println!("Found {} reports", reports.len());
}
