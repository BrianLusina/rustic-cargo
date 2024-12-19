#[cfg(test)]
mod is_palindrome_tests {
    use strings::palindrome::is_palindrome::is_palindrome;

    #[test]
    fn test_anna() {
        let word = "anna";
        let actual = is_palindrome(word);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_walter() {
        let word = "walter";
        let actual = is_palindrome(word);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_12321() {
        let word = "12321";
        let actual = is_palindrome(word);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_123456() {
        let word = "123456";
        let actual = is_palindrome(word);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_was_it_a_cat_i_saw() {
        let word = "Was it a cat I saw?";
        let actual = is_palindrome(word);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_12() {
        let word = "Never odd or even";
        let actual = is_palindrome(word);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_13() {
        let word = "radar";
        let actual = is_palindrome(word);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_14() {
        let word = "Live on time, emit no evil";
        let actual = is_palindrome(word);
        assert_eq!(actual, true);
    }
}
