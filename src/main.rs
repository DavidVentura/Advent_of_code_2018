pub mod aoc_1;

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

fn main() {
    let body = get_challenge_data(1);
    let data : Vec<i32> = body.split('\n').map(|x| x.parse().unwrap()).collect();
    aoc_1::part_1(&data);
    let result_2 = aoc_1::part_2(&data);
    println!("Result part 2: {}", result_2);
}

