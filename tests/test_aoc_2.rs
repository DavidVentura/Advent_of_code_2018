#[cfg(test)]
mod tests {
    use aoc::aoc_2;
    #[test]
    fn test_part_1() {
        let data: Vec<String> = [
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
        ]
        .to_vec();
        assert_eq!(aoc_2::part_1(&data), 12);
    }
}
