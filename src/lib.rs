use reqwest::{blocking::Client, header::COOKIE};

pub fn fetch_aoc_input(year: u32, day: u32) -> reqwest::Result<String> {
    let session =
        std::env::var("AOC_SESSION").expect("AOC_SESSION environment variable must be set");

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let client = Client::builder().build()?;

    let response = client
        .get(&url)
        .header(COOKIE, format!("session={session}"))
        .send()?;

    response.text()
}
