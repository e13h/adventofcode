use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs;
use std::path::Path;

pub mod aoc {
    use super::*;
    /// Configuration for Advent of Code puzzle fetching
    pub struct AocConfig {
        year: u16,
        session_cookie: String,
        download_path: String,
    }

    impl AocConfig {
        /// Create a new AocConfig from environment variables
        pub fn new() -> Result<Self> {
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

    pub fn get_aoc_puzzle_input(day: u8) -> Result<String> {
        // Load configuration
        let config = AocConfig::new()?;

        // Create download directory if it doesn't exist
        fs::create_dir_all(&config.download_path)?;

        // Construct input file path
        let input_path = Path::new(&config.download_path).join(format!("day{:02}.txt", day));

        // Check if input is already downloaded
        if input_path.exists() {
            return Ok(fs::read_to_string(&input_path)?);
        }

        // Create HTTP client
        let client = Client::new();

        // Fetch puzzle input
        let url = format!("https://adventofcode.com/{}/day/{}/input", config.year, day);

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
        let input_text = response.text().context("Failed to read response text")?;

        // Save input to file
        fs::write(&input_path, &input_text)?;

        Ok(input_text)
    }
}
