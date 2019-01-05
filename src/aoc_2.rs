use std::collections::HashMap;

fn part_1(data: &Vec<String>) -> u32 {
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;

    for item in data {
        let mut letter_count: HashMap<char, u8> = HashMap::new();

        for c in item.chars() {
            let value = match letter_count.get(&c) {
                Some(_count) => _count + 1,
                None => 1,
            };
            letter_count.insert(c, value);
        }
        for val in letter_count.values() {
            if *val == 2 {
                twos += 1;
                break;
            }
        }
        for val in letter_count.values() {
            if *val == 3 {
                threes += 1;
                break;
            }
        }
    }
    println!(
        "Twos: {}, Threes: {}, Result: {}",
        twos,
        threes,
        twos * threes
    );
    twos * threes
}

fn part_2(data: &Vec<String>) -> i32 {
    5
}

pub fn main() {
    let body = super::get_challenge_data(2);
    let data: Vec<String> = body.split('\n').map(|x| x.to_string()).collect();
    part_1(&data);
}
