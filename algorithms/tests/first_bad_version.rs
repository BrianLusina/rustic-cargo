#[cfg(test)]
mod first_bad_version_tests {
    use algorithms::binarysearch::firstbadversion::first_bad_version::Solution;

    #[test]
    fn test_1() {
        let solution = Solution::new(4);
        let actual = solution.first_bad_version(5);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let solution = Solution::new(1);
        let actual = solution.first_bad_version(1);
        let expected = 1;
        assert_eq!(actual, expected);
    }
}
