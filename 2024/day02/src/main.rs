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

fn is_monotonic(comparison: impl Fn(i32, i32) -> bool, vec: &[i32], max_step: Option<i32>) -> bool {
    let max_step = max_step.unwrap_or(i32::MAX);
    let mut prev = i32::MIN;
    for (i, &value) in vec.iter().enumerate() {
        if i > 0 && (comparison(value, prev) || ((value - prev).abs() > max_step)) {
            return false
        }
        prev = value;
    }
    return true
}

fn is_strictly_increasing(vec: &[i32], max_step: Option<i32>) -> bool {
    is_monotonic(|a, b| a <= b, vec, max_step)
}

fn is_strictly_decreasing(vec: &[i32], max_step: Option<i32>) -> bool {
    is_monotonic(|a, b| a >= b, vec, max_step)
}

fn report_is_safe(report: &[i32]) -> bool {
    is_strictly_increasing(report, Some(3)) || is_strictly_decreasing(report, Some(3))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_report_is_safe() {
        // Test cases provided by https://adventofcode.com/2024/day/2
        assert_eq!(report_is_safe(&[7, 6, 4, 2, 1]), true);
        assert_eq!(report_is_safe(&[1, 3, 6, 7, 9]), true);
    }
    
    #[test]
    fn test_invalid_report_is_unsafe() {
        assert_eq!(report_is_safe(&[1, 2, 7, 8, 9]), false);
        assert_eq!(report_is_safe(&[9, 7, 6, 2, 1]), false);
        assert_eq!(report_is_safe(&[1, 3, 2, 4, 5]), false);
        assert_eq!(report_is_safe(&[8, 6, 4, 4, 1]), false);
    }
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for report in reports {
        if report_is_safe(&report) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = get_aoc_puzzle_input(2).unwrap();
    let reports = parse_input(input).unwrap();
    println!("{}/{} reports are safe", count_safe_reports(&reports), reports.len())
}
