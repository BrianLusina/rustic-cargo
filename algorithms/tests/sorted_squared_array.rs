#[cfg(test)]
mod sorted_squared_array_tests {
    use algorithms::arrays::sortedsquaredarray::sorted_squared_array::sorted_squares;

    #[test]
    fn test_1() {
        let numbers: Vec<i32> = vec![-4,-1,0,3,10];
        let expected = vec![0,1,9,16,100];
        let actual = sorted_squares(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let numbers: Vec<i32> = vec![-7,-3,2,3,11];
        let expected = vec![4,9,9,49,121];
        let actual = sorted_squares(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all_positive() {
        let numbers: Vec<i32> = vec![1, 2, 3, 5, 6, 8, 9];
        let expected = vec![1, 4, 9, 25, 36, 64, 81];
        let actual = sorted_squares(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all_negative() {
        let numbers: Vec<i32> = vec![-9, -8, -6, -5, -3, -2, -1];
        let expected = vec![1, 4, 9, 25, 36, 64, 81];
        let actual = sorted_squares(numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_single_element() {
        let numbers: Vec<i32> = vec![5];
        let expected = vec![25];
        let actual = sorted_squares(numbers);
        assert_eq!(actual, expected);
    }
}
