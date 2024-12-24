use aoc_getter::aoc::get_aoc_puzzle_input;
use std::result::Result;
use std::collections::VecDeque;

fn parse_input(puzzle_input: String) -> Result<(Vec<i32>, Vec<i32>), String> {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for (i, line) in puzzle_input.lines().enumerate() {
        // collect() turns an iterator of Result<T, E> items into Result<Collection<T>, E>
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() != 2 {
            return Err(format!(
                "Line {} does not have exactly 2 columns: '{}'",
                i + 1,
                line
            ));
        }
        let num1 = parts[0].trim().parse::<i32>().map_err(|e| {
            format!(
                "Error parsing first number on line {}: '{}' - {}",
                i + 1,
                parts[0],
                e
            )
        })?;
        let num2 = parts[1].trim().parse::<i32>().map_err(|e| {
            format!(
                "Error parsing first number on line {}: '{}' - {}",
                i + 1,
                parts[1],
                e
            )
        })?;
        col1.push(num1);
        col2.push(num2);
    }
    Ok((col1, col2))
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut left: VecDeque<i32> = VecDeque::from(left);
    let mut right: VecDeque<i32> = VecDeque::from(right);
    let mut result = Vec::new();
    while (left.len() > 0) & (right.len() > 0) {
        if left[0] <= right[0] {
            result.push(left.pop_front().unwrap());
        } else {
            result.push(right.pop_front().unwrap());
        }
    }
    let mut left = Vec::from(left);
    let mut right = Vec::from(right);
    result.append(&mut left);
    result.append(&mut right);
    result
}

fn merge_sort(input_array: Vec<i32>) -> Vec<i32> {
    if input_array.len() <= 1 {
        return input_array;
    }
    let left = input_array[..(input_array.len() / 2)].to_vec();
    let right = input_array[(input_array.len() / 2)..].to_vec();
    let left = merge_sort(left);
    let right = merge_sort(right);
    merge(left, right)
}

fn verify_sorted(input: Vec<i32>) -> bool {
    let mut prev = 0;
    for curr in input {
        if prev > curr {
            return false
        }
        prev = curr
    }
    return true
}

fn main() {
    let input = get_aoc_puzzle_input(1).unwrap();
    let (col1, col2) = parse_input(input).unwrap();
    println!("sorted? {}", verify_sorted(col1.clone()));
    println!("sorted? {}", verify_sorted(col2.clone()));
    let col1 = merge_sort(col1);
    let col2 = merge_sort(col2);
    println!("sorted? {}", verify_sorted(col1));
    println!("sorted? {}", verify_sorted(col2));
}
