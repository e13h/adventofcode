use aoc_getter::aoc::get_aoc_puzzle_input;
use regex::Regex;

enum Part {
    One,
    Two
}

fn parse_input(puzzle_input: &str, part: Part) -> (Vec<i32>, Vec<i32>) {
    let re = match part {
        Part::One => Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(),
        Part::Two => panic!("Part 2 regex not yet defined!"),
    };
    re.captures_iter(&puzzle_input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            [x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()]
        })
        .map(|[x, y]| (x, y))
        .unzip()
}

fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    let mut result = 0;
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input_parser() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (a, b) = parse_input(test_input, Part::One);
        assert_eq!(a, [2, 5, 11, 8]);
        assert_eq!(b, [4, 5, 8, 5]);
    }

    #[test]
    fn test_dot_product() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (a, b) = parse_input(test_input, Part::One);
        assert_eq!(dot_product(&a, &b), 161);
    }

    #[test]
    fn test_updated_puzzle_input_parser() {
        let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (a, b) = parse_input(test_input, Part::Two);
        assert_eq!(a, [2, 8]);
        assert_eq!(b, [4, 5]);
    }
}

fn main() {
    let input = get_aoc_puzzle_input(3).unwrap();
    let (a, b) = parse_input(&input, Part::One);
    let result = dot_product(&a, &b);
    println!("Part 1 result is {result}");
    let (a, b) = parse_input(&input, Part::Two);
    let result = dot_product(&a, &b);
    println!("Part 2 result is {result}")
}
