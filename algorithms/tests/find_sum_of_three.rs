#[cfg(test)]
mod find_sum_of_three_tests {
    use algorithms::twopointers::findsumofthree::find_sum_of_three::find_sum_of_three;

    #[test]
    fn test_1() {
        let numbers = &mut [1, -1, 0];
        let target = -1;
        let expected = false;
        let actual = find_sum_of_three(numbers, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let numbers = &mut [3, 7, 1, 2, 8, 4, 5];
        let target = 21;
        let expected = false;
        let actual = find_sum_of_three(numbers, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let numbers = &mut [3, 7, 1, 2, 8, 4, 5];
        let target = 10;
        let expected = true;
        let actual = find_sum_of_three(numbers, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let numbers = &mut [-1, 2, 1, -4, 5, -3];
        let target = -8;
        let expected = true;
        let actual = find_sum_of_three(numbers, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let numbers = &mut [-1, 2, 1, -4, 5, -3];
        let target = 0;
        let expected = true;
        let actual = find_sum_of_three(numbers, target);
        assert_eq!(actual, expected);
    }
}
