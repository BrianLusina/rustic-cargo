#[cfg(test)]
mod is_anagram_tests {
    use strings::anagram::anagrams::is_anagram;

    #[test]
    fn fairy_tales_and_rail_safety() {
        let word1 = "fairy tales";
        let word2 = "rail safety";
        let actual = is_anagram(word1, word2);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn william_shakespeare_and_i_am_a_weakish_speller() {
        let word1 = "William Shakespeare";
        let word2 = "I am a weakish speller";
        let actual = is_anagram(word1, word2);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn madam_curie_and_radium_came() {
        let word1 = "Madam Curie";
        let word2 = "Radium came";
        let actual = is_anagram(word1, word2);
        let expected = true;
        assert_eq!(actual, expected);
    }
}
