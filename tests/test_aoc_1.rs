#[cfg(test)]
mod tests {
    use aoc::aoc_1;
    #[test]
    fn test_part_2() {
        assert_eq!(aoc_1::part_2(&[1, -1].to_vec()), 0);
        assert_eq!(aoc_1::part_2(&[3, 3, 4, -2, -4].to_vec()), 10);
        assert_eq!(aoc_1::part_2(&[-6, 3, 8, 5, -6].to_vec()), 5);
        assert_eq!(aoc_1::part_2(&[7, 7, -2, -7, -4].to_vec()), 14);
    }
}
