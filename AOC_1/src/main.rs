use reqwest;
use reqwest::header::COOKIE;
use std::env;

fn main() {
    let challenge_url = "https://adventofcode.com/2018/day/1/input";
    let session = env::var("AOC_SESSION").unwrap();
    let client = reqwest::Client::new();
    let body = client
        .get(challenge_url)
        .header(COOKIE, String::from("session=".to_owned() + &session))
        .send().unwrap().text().unwrap();
    let body = body.trim();
    assert!(body.contains("Puzzle inputs differ by user") == false);
    let list: Vec<i32> = body.split('\n').map(|x| x.parse().unwrap()).collect();
    let result: i32 = list.iter().sum();
    println!("{:?}", list);
    println!("{}", result);
}
