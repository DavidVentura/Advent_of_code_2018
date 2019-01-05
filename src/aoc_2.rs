use std::collections::HashMap;

pub fn part_1(data: &Vec<String>) -> u32 {
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

pub fn distance(word1: &str, word2: &str) -> u32 {
    assert_eq!(word1.len(), word2.len());
    let mut wchars1 = word1.chars();
    let mut wchars2 = word2.chars();
    let mut distance = 0;
    for i in 0..word1.len() {
        let c1 = wchars1.next();
        let c2 = wchars2.next();
        if c1 != c2 {
            distance += 1;
        }
    }
    distance
}
pub fn common_chars(word1: &str, word2: &str) -> String {
    assert_eq!(word1.len(), word2.len());
    let mut wchars1 = word1.chars();
    let mut wchars2 = word2.chars();
    let mut word: String = String::new();
    for i in 0..word1.len() {
        let c1 = wchars1.next().unwrap();
        let c2 = wchars2.next().unwrap();
        if c1 == c2 {
            word.push(c1);
        }
    }
    word
}

pub fn closest_word(data: &Vec<String>, word: &str) -> String {
    let mut _distance = std::u32::MAX;
    let mut ret: &str = word;
    for w in data {
        if w == word {
            continue;
        }
        let d = distance(w, word);
        if d < _distance {
            _distance = d;
            ret = w;
        }
    }
    ret.to_string()
}

pub fn part_2(data: &Vec<String>) -> u32 {
    let closest_words = data.iter().map(|w| (w, closest_word(data, w)));
    println!("{:?}", closest_words);
    let distances = closest_words.map(|(w, c)| (w, c.clone(), distance(w, &c)));
    println!("{:?}", distances);
    0
}

pub fn main() {
    let body = super::get_challenge_data(2);
    let data: Vec<String> = body.split('\n').map(|x| x.to_string()).collect();
    //part_1(&data);
    //println!("{}", );
    //println!("{}",
    part_2(&data);
}
