fn power_of_two(n: u32) -> bool {
    n > 0 && n & (n - 1) == 0
}

#[cfg(test)]
mod power_of_two_tests {
    use super::*;

    #[test]
    fn test_power_of_two() {
        assert_eq!(power_of_two(1), true);
        assert_eq!(power_of_two(2), true);
        assert_eq!(power_of_two(3), false);
    }
}
