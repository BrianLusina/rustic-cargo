// returns the first word in a given string slice reference.
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod tests {
    use crate::utils::first_word::first_word;

    #[test]
    fn first_word_test() {
        assert_eq!(first_word("hello world"), "hello");
    }
}
