use anyhow::{bail, Result};
use reqwest;
use std::fs::read_to_string;
use std::path::Path;

static AOC_SESSION_FILE: &'static str = ".aoc-session";

/// This function downloads input data from Advent of Code
/// if .aoc-session file is available and download succeeds.
pub fn get_input_aoc(url: &str) -> Result<String> {
    let f = Path::new(&AOC_SESSION_FILE);

    if !f.is_file() {
        bail!("{:?} not found.", &AOC_SESSION_FILE);
    }

    // Load session key
    let session_key = read_to_string(f)?;

    // Load input data
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_key))
        .send()
        .expect("Sending request failed.");

    if response.status().is_success() {
        let resp = response.text()?;
        println!("! Loaded input data from: {}", url);
        return Ok(resp);
    } else {
        bail!(
            "Failed to load {:?}. Response: {:?}",
            url,
            response.status()
        )
    }
}
