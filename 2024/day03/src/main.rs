use aoc_getter::aoc::get_aoc_puzzle_input;
use regex::Regex;

fn parse_input(puzzle_input: String) -> Vec<[i32; 2]> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&puzzle_input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            [x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()]
        })
        .collect()
}

fn main() {
    let input = get_aoc_puzzle_input(3).unwrap();
    let instructions = parse_input(input);
    println!("{instructions:?}");
}
