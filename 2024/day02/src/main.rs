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

fn is_monotonic(
    breaks_monotonicity: &impl Fn(i32, i32) -> bool,
    vec: &[i32],
    max_step: Option<i32>,
    skip_errors: Option<i32>,
) -> bool {
    let max_step = max_step.unwrap_or(i32::MAX);
    let skip_errors = skip_errors.unwrap_or(0);
    let mut prev: Option<i32> = None;
    if vec.len() <= 1 {
        return true;
    }
    for (i, &value) in vec.iter().enumerate() {
        let level_is_unsafe = i > 0
            && (breaks_monotonicity(value, prev.unwrap())
                || (value - prev.unwrap()).abs() > max_step);
        if level_is_unsafe && (skip_errors > 0) {
            // Try removing previous level and checking for safety
            if is_monotonic(
                breaks_monotonicity,
                &[&vec[..i - 1], &vec[i..]].concat(),
                Some(max_step),
                Some(skip_errors - 1),
            ) {
                return true;
            }
            // Try removing current level and checking for safety
            if is_monotonic(
                breaks_monotonicity,
                &[&vec[..i], &vec[i + 1..]].concat(),
                Some(max_step),
                Some(skip_errors - 1),
            ) {
                return true;
            }
            return false;
        } else if level_is_unsafe {
            return false;
        }
        prev = Some(value);
    }
    return true;
}

fn is_strictly_increasing(vec: &[i32], max_step: Option<i32>, skip_errors: Option<i32>) -> bool {
    is_monotonic(&|a, b| a <= b, vec, max_step, skip_errors)
}

fn is_strictly_decreasing(vec: &[i32], max_step: Option<i32>, skip_errors: Option<i32>) -> bool {
    is_monotonic(&|a, b| a >= b, vec, max_step, skip_errors)
}

fn report_is_safe(report: &[i32], problem_dampener: Option<i32>) -> bool {
    let problem_dampener = problem_dampener.unwrap_or(0);
    is_strictly_increasing(report, Some(3), Some(problem_dampener))
        || is_strictly_decreasing(report, Some(3), Some(problem_dampener))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_report_is_safe() {
        // Test cases provided by https://adventofcode.com/2024/day/2
        assert_eq!(report_is_safe(&[7, 6, 4, 2, 1], None), true);
        assert_eq!(report_is_safe(&[1, 3, 6, 7, 9], None), true);
    }

    #[test]
    fn test_invalid_report_is_unsafe() {
        assert_eq!(report_is_safe(&[1, 2, 7, 8, 9], None), false);
        assert_eq!(report_is_safe(&[9, 7, 6, 2, 1], None), false);
        assert_eq!(report_is_safe(&[1, 3, 2, 4, 5], None), false);
        assert_eq!(report_is_safe(&[8, 6, 4, 4, 1], None), false);

        assert_eq!(report_is_safe(&[2, 1, 2, 3, 4], None), false);
        assert_eq!(report_is_safe(&[1, 2, 3, 4, 2], None), false);
    }

    #[test]
    fn test_invalid_reports_continue_to_be_safe_with_problem_dampener() {
        assert_eq!(report_is_safe(&[7, 6, 4, 2, 1], Some(1)), true);
        assert_eq!(report_is_safe(&[1, 3, 6, 7, 9], Some(1)), true);
    }

    #[test]
    fn test_invalid_reports_become_safe_with_problem_dampener() {
        assert_eq!(report_is_safe(&[1, 3, 2, 4, 5], Some(1)), true);
        assert_eq!(report_is_safe(&[8, 6, 4, 4, 1], Some(1)), true);

        assert_eq!(report_is_safe(&[2, 1, 2, 3, 4], Some(1)), true);
        assert_eq!(report_is_safe(&[1, 2, 3, 4, 1], Some(1)), true);
    }

    #[test]
    fn test_invalid_reports_remain_unsafe_with_problem_dampener() {
        assert_eq!(report_is_safe(&[1, 2, 7, 8, 9], Some(1)), false);
        assert_eq!(report_is_safe(&[9, 7, 6, 2, 1], Some(1)), false);
    }
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for report in reports {
        if report_is_safe(&report, Some(1)) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = get_aoc_puzzle_input(2).unwrap();
    let reports = parse_input(input).unwrap();
    println!(
        "{}/{} reports are safe",
        count_safe_reports(&reports),
        reports.len()
    )
}
