use aoc_getter::aoc::get_aoc_puzzle_input;
use std::result::Result;

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

// fn merge_sort(input_array: Vec<i32>) -> Vec<i32> {
//     println!("{:#?}", input_array);
//     // println!("{}", input_array[0]);
//     input_array
// }

fn main() {
    println!("Hello, world!");
    match get_aoc_puzzle_input(1) {
        Ok(input) => {
            let (col1, col2) = parse_input(input).unwrap();
            println!("{:#?}", &col1[0..5]);
            println!("{:#?}", &col2[0..5]);
            // merge_sort(parse_input(input).unwrap());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
