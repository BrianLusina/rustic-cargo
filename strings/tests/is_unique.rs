#[cfg(test)]
mod is_unique_tests {
    use strings::utils::is_unique::is_unique;

    #[test]
    fn empty_string() {
        let input = "";
        let actual = is_unique(input);
        let expected = true;
        assert_eq!(expected, actual);
    }

    #[test]
    fn abCDefGh_should_return_true() {
        let input = "abCDefGh";
        let actual = is_unique(input);
        let expected = true;
        assert_eq!(expected, actual);
    }

    #[test]
    fn nonunique_should_return_false() {
        let input = "nonunique";
        let actual = is_unique(input);
        let expected = false;
        assert_eq!(expected, actual);
    }

    #[test]
    fn abCedFghI_should_return_true() {
        let input = "abCedFghI";
        let actual = is_unique(input);
        let expected = true;
        assert_eq!(expected, actual);
    }

    #[test]
    fn I_Am_Not_Unique_should_return_false() {
        let input = "I Am Not Unique";
        let actual = is_unique(input);
        let expected = false;
        assert_eq!(expected, actual);
    }

    #[test]
    fn heythere_should_return_false() {
        let input = "heythere";
        let actual = is_unique(input);
        let expected = false;
        assert_eq!(expected, actual);
    }

    #[test]
    fn hi_should_return_false() {
        let input = "hi";
        let actual = is_unique(input);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
