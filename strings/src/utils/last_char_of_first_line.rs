// Returns the last char of the first line of a string
// This function returns Option<char> because it’s possible that there is a character there,
// but it’s also possible that there isn’t. This code takes the text string slice argument and
// calls the lines method on it, which returns an iterator over the lines in the string.
// Because this function wants to examine the first line, it calls next on the iterator to get
// the first value from the iterator. If text is the empty string, this call to next will return
// None, in which case we use ? to stop and return None from last_char_of_first_line.
// If text is not the empty string, next will return a Some value containing a string slice of
// the first line in text
pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(last_char_of_first_line("hello"), Some('o'));
    }

    #[test]
    fn test_leading_blank_line() {
        assert_eq!(last_char_of_first_line("\nhello"), None);
    }
}
