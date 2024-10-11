fn factorial(number: u32) -> u32 {
    if number == 0 {
        1
    } else {
        number * factorial(number-1)
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;

    #[test]
    fn factorial_of_5() {
        let result = factorial(5);
        let expected = 120;
        assert_eq!(result, expected);
    }
}
