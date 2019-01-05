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
    #[test]
    #[should_panic]
    fn test_distance_fails_length() {
        aoc_2::distance("abc", "abcd");
    }
    #[test]
    fn test_distance() {
        assert_eq!(aoc_2::distance("abc", "abc"), 0);
        assert_eq!(aoc_2::distance("abc", "abb"), 1);
        assert_eq!(aoc_2::distance("abc", "acb"), 2);
        assert_eq!(aoc_2::distance("abc", "cab"), 3);
    }

    #[test]
    fn test_common_chars() {
        assert_eq!(aoc_2::common_chars("abc", "acc"), "ac");
    }

    #[test]
    fn test_closest_word() {}
}
