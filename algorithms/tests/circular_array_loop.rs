#[cfg(test)]
mod circular_array_loop_tests {
    use algorithms::fastandslow::circulararrayloop::circular_array_loop::circular_array_loop;

    #[test]
    fn test_1() {
        let numbers: Vec<i32> = vec![3, 1, 2];
        let expected = true;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let numbers = vec![-2, -1, -3];
        let expected = true;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let numbers = vec![2, 1, -1, -2];
        let expected = false;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let numbers = vec![3, -3, 1, 1];
        let expected = true;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let numbers = vec![1, 3, -2, -4, 1];
        let expected = true;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let numbers = vec![5, 4, -2, -1, 3];
        let expected = false;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let numbers = vec![1, 2, -3, 3, 4, 7, 1];
        let expected = true;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let numbers = vec![3, 3, 1, -1, 2];
        let expected = true;
        let actual = circular_array_loop(numbers);
        assert_eq!(actual, expected);
    }
}
