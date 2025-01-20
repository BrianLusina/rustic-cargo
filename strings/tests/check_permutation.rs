#[cfg(test)]
mod check_permutation_with_sorting_tests {
    use strings::permutation::check_permutation::check_permutation_with_sorting;

    #[test]
    fn google_and_ooggle_test() {
        let input_1 = "google";
        let input_2 = "ooggle";
        let expected = true;
        let actual = check_permutation_with_sorting(input_1, input_2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn not_and_top_test() {
        let input_1 = "not";
        let input_2 = "top";
        let expected = false;
        let actual = check_permutation_with_sorting(input_1, input_2);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod check_permutation_with_map_tests {
    use strings::permutation::check_permutation::check_permutation_with_map;

    #[test]
    fn google_and_ooggle_test() {
        let input_1 = "google";
        let input_2 = "ooggle";
        let expected = true;
        let actual = check_permutation_with_map(input_1, input_2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn not_and_top_test() {
        let input_1 = "not";
        let input_2 = "top";
        let expected = false;
        let actual = check_permutation_with_map(input_1, input_2);
        assert_eq!(actual, expected);
    }
}
