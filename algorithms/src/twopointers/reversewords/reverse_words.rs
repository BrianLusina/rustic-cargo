pub fn reverse_words(s: String) -> String {
    let mut words: Vec<&str> = s.split_whitespace().collect();

    if words.len() == 1 {
        return words.join("");
    }

    let mut left = 0;
    let mut right = words.len() - 1;

    while left <= right {
        words.swap(left, right);
        left += 1;
        right -= 1;
    }

    words.join(" ")
}

#[cfg(test)]
mod reverse_words_tests {
    use super::reverse_words;
    use parameterized::parameterized;
    use parameterized::ide;

    ide!();

    #[parameterized(sentence = {
        "the sky is blue",
        "  hello world  ",
        "a good   example",
        "EPY2giL",
        "t "
    }, expected = {
        "blue is sky the",
        "world hello",
        "example good a",
        "EPY2giL",
        "t"
    })]
    fn tests(sentence: &str, expected: &str) {
        let actual = reverse_words(sentence.to_string());
        assert_eq!(expected, actual);
    }
}