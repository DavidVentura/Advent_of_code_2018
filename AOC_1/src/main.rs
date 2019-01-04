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
    let session = env::var("AOC_SESSION").unwrap();
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
    let list: Vec<i32> = body.split('\n').map(|x| x.parse().unwrap()).collect();
    let result: i32 = list.iter().sum();
    println!("{:?}", list);
    println!("{}", result);
}
