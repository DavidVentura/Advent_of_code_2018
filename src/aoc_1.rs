use std::collections::BTreeSet;
pub fn part_1(data: &Vec<i32>) {
    let result: i32 = data.iter().sum();
    println!("{}", result);
}

pub fn part_2(data: &Vec<i32>) -> i32 {
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
