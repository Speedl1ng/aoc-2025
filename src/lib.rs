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

pub fn next_even_len_up(n: &str) -> u64 {
    let len = n.len();
    if len.is_multiple_of(2) {
        n.parse().unwrap()
    } else {
        10u64.pow(len as u32)
    }
}

pub fn next_even_len_down(n: &str) -> u64 {
    let len = n.len();
    if len.is_multiple_of(2) {
        n.parse().unwrap()
    } else {
        10u64.pow((len - 1) as u32) - 1
    }
}

fn is_palindrome(n: u64) -> bool {
    let mut reverse = 0;
    let mut temp = n;
    while temp != 0 {
        reverse = (reverse * 10) + (temp % 10);
        temp /= 10;
    }
    reverse == n
}

#[cfg(test)]
mod test {
    use crate::{is_palindrome, next_even_len_down, next_even_len_up};

    #[test]
    fn up() {
        assert_eq!(next_even_len_up(&9.to_string()), 10);
        assert_eq!(next_even_len_up(&159.to_string()), 1000);
        assert_eq!(next_even_len_up(&15917.to_string()), 100000);
    }
    #[test]
    fn down() {
        assert_eq!(next_even_len_down(&9.to_string()), 0);
        assert_eq!(next_even_len_down(&159.to_string()), 99);
        assert_eq!(next_even_len_down(&15917.to_string()), 9999);
    }
    #[test]
    fn palin() {
        assert!(is_palindrome(11));
        assert!(!is_palindrome(12));
        assert!(is_palindrome(446446));
    }
}
