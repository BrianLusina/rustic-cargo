/**
* Sorts a string and returns the string sorted
**/
pub fn sort_string_unstable(s: &str) -> String {
    let mut char_collection: Vec<char> = s.chars().collect();
    char_collection.sort_unstable();
    char_collection.into_iter().collect()
}

#[cfg(test)]
mod sort_word_tests {
    use crate::utils::sort_word::sort_string_unstable;

    #[test]
    fn test_sort() {
        let word = "bacd";
        let expected = "abcd";
        let actual = sort_string_unstable(word);
        assert_eq!(expected, actual);
    }
}
