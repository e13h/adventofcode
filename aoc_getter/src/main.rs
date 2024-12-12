use anyhow::{Context, Result};
use clap::Parser;
use reqwest::blocking::Client;
use std::fs;
use std::path::Path;

/// CLI arguments for Advent of Code puzzle input fetcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of Advent of Code puzzle to fetch (1-25)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}

/// Configuration for Advent of Code puzzle fetching
struct AocConfig {
    year: u16,
    session_cookie: String,
    download_path: String,
}

impl AocConfig {
    /// Create a new AocConfig from environment variables
    fn new() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            year: std::env::var("AOC_YEAR")
                .context("AOC_YEAR not set")?
                .parse()
                .context("Invalid AOC_YEAR")?,
            session_cookie: std::env::var("AOC_SESSION")
                .context("AOC_SESSION cookie not set")?,
            download_path: std::env::var("AOC_DOWNLOAD_PATH")
                .unwrap_or_else(|_| "./inputs".to_string()),
        })
    }
}

fn get_aoc_puzzle_input(day: u8) -> Result<String> {
    // Load configuration
    let config = AocConfig::new()?;

    // Create download directory if it doesn't exist
    fs::create_dir_all(&config.download_path)?;

    // Construct input file path
    let input_path = Path::new(&config.download_path)
        .join(format!("day{:02}.txt", day));

    // Check if input is already downloaded
    if input_path.exists() {
        return Ok(fs::read_to_string(&input_path)?);
    }

    // Create HTTP client
    let client = Client::new();

    // Fetch puzzle input
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        config.year,
        day
    );

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", config.session_cookie))
        .send()
        .context("Failed to send request")?;

    // Check response status
    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch input. Status: {}", response.status());
    }

    // Get input text
    let input_text = response
        .text()
        .context("Failed to read response text")?;

    // Save input to file
    fs::write(&input_path, &input_text)?;

    Ok(input_text)
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
