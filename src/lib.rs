pub mod aoc_1;
pub mod aoc_2;

use reqwest;
use reqwest::header::COOKIE;
use std::env;
use url::Url;

fn get_challenge_data(number: u8) -> String {
    let url = format!(
        "https://adventofcode.com/2018/day/{}/input",
        number.to_string()
    );
    let challenge_url = Url::parse(&url).unwrap();
    let session = env::var("AOC_SESSION").expect("You have not set the AOC_SESSION environment variable");
    let client = reqwest::Client::new();
    let body = client
        .get(challenge_url)
        .header(COOKIE, String::from("session=".to_owned() + &session))
        .send()
        .unwrap()
        .text()
        .unwrap();
    let body = body.trim();
    assert!(body.contains("Puzzle inputs differ by user") == false);
    body.to_string()
}
