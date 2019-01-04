use std::collections::BTreeSet;
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

fn part_1(data: &Vec<i32>) {
    let result: i32 = data.iter().sum();
    println!("{}", result);
}

fn part_2(data: &Vec<i32>) -> i32 {
    //let mut totals: Vec<i32> = Vec::new();
    let mut totals = BTreeSet::new();
    let mut acum: i32 = 0;
    let mut i: u32 = 0;
    loop {
        if totals.contains(&acum) {
            return acum;
        }
        //totals.push(acum);
        totals.insert(acum);
        acum += data[i as usize];
        i += 1;
        i = i % (data.len() as u32);
    }
}

fn main() {
    let body = get_challenge_data(1);
    let data : Vec<i32> = body.split('\n').map(|x| x.parse().unwrap()).collect();
    part_1(&data);
    let result_2 = part_2(&data);
    println!("Result part 2: {}", result_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_2() {
        assert_eq!(part_2([1, -1].to_vec()), 0);
        assert_eq!(part_2([3, 3, 4, -2, -4].to_vec()), 10);
        assert_eq!(part_2([-6, 3, 8, 5, -6].to_vec()), 5);
        assert_eq!(part_2([7, 7, -2, -7, -4].to_vec()), 14);
    }
}
