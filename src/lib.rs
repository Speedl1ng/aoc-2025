use reqwest::{blocking::Client, header::COOKIE};

#[allow(clippy::missing_panics_doc)]
pub fn fetch_aoc_input(day: u8) -> String {
    let session =
        std::env::var("AOC_SESSION").expect("AOC_SESSION environment variable must be set");

    let url = format!("https://adventofcode.com/2025/day/{day}/input");

    let response = Client::new()
        .get(&url)
        .header(COOKIE, format!("session={session}"))
        .send()
        .expect("Failed to fetch input");

    response.text().expect("Failed to read response text")
}
