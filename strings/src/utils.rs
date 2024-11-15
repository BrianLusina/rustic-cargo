/**
* Sorts a string and returns the string sorted
**/
pub fn sort_word(word: &str) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();
    sorted.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::sort_word;

    #[test]
    fn test_sort() {
        let word = "bacd";
        let expected = "abcd";
        let actual = sort_word(word);
        assert_eq!(expected, actual);
    }
}
