fn factorial(number: u32) -> u32 {
    if number == 0 {
        1
    } else {
        number * factorial(number - 1)
    }
}

fn factorial_iter(target: u32) -> u32 {
    if target == 0 {
        1
    } else {
        let mut start = 1;
        let mut result = 1;

        while start <= target {
            result *= start;
            start += 1;
        }

        result
    }
}

fn factorial_iter_with_for_loop(x: u32) -> u32 {
    if x == 0 {
        return 1;
    }
    let mut start = 1;
    let mut result = 1;

    for num in start..=x {
        result *= start;
        start += 1;
    }
    result
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

    #[test]
    fn factorial_of_5_iter() {
        let result = factorial_iter(5);
        let expected = 120;
        assert_eq!(result, expected);
    }

    #[test]
    fn factorial_of_5_iter_for_loop() {
        let result = factorial_iter_with_for_loop(5);
        let expected = 120;
        assert_eq!(result, expected);
    }
}
