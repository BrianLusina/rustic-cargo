fn name_shuffler(s: &str) -> String {
    // Preserve the exact whitespace between the names (and also keep any leading/trailing whitespace)
    // Identify: leading_ws, first, middle_ws, second, trailing_ws and then reassemble as:
    // leading_ws + second + middle_ws + first + trailing_ws
    // If we can't find two "words", return the original string unchanged.
    // Note: "word" here means a maximal run of non-whitespace characters.
    // Use char_indices to stay on UTF-8 boundaries.
    let len = s.len();

    // Find start of first token (skip leading whitespace).
    let mut first_start_opt = None;
    for (idx, ch) in s.char_indices() {
        if !ch.is_whitespace() {
            first_start_opt = Some(idx);
            break;
        }
    }
    let Some(first_start) = first_start_opt else {
        // No non-whitespace characters at all; return as-is.
        return s.to_string();
    };

    // Find end of first token.
    let mut first_end = len;
    for (off, ch) in s[first_start..].char_indices() {
        if ch.is_whitespace() {
            first_end = first_start + off;
            break;
        }
    }
    if first_end == len {
        // Only one token present.
        return s.to_string();
    }

    // Find start of second token (skip the middle whitespace).
    let mut second_start = len;
    for (off, ch) in s[first_end..].char_indices() {
        if !ch.is_whitespace() {
            second_start = first_end + off;
            break;
        }
    }
    if second_start == len {
        // No second token.
        return s.to_string();
    }

    // Find end of second token.
    let mut second_end = len;
    for (off, ch) in s[second_start..].char_indices() {
        if ch.is_whitespace() {
            second_end = second_start + off;
            break;
        }
    }
    if second_end == len {
        // token runs to end
        second_end = len;
    }

    let leading = &s[..first_start];
    let first = &s[first_start..first_end];
    let middle = &s[first_end..second_start];
    let second = &s[second_start..second_end];
    let trailing = &s[second_end..];

    let mut out = String::with_capacity(s.len());
    out.push_str(leading);
    out.push_str(second);
    out.push_str(middle);
    out.push_str(first);
    out.push_str(trailing);
    out
}

#[cfg(test)]
mod name_shuffler_string_tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn an_empty_string() {
        let input = "";
        let output = name_shuffler(input);
        let expected = "";
        assert_eq!(output, expected);
    }

    #[test]
    fn john_mcclane() {
        let input = "john McClane";
        let output = name_shuffler(input);
        let expected = "McClane john";
        assert_eq!(output, expected);
    }

    #[test]
    fn mary_jeggins() {
        let input = "Mary jeggins";
        let output = name_shuffler(input);
        let expected = "jeggins Mary";
        assert_eq!(output, expected);
    }

    #[test]
    fn tom_jerry() {
        let input = "tom jerry";
        let output = name_shuffler(input);
        let expected = "jerry tom";
        assert_eq!(output, expected);
    }

    // Additional tests to ensure whitespace is preserved and edge cases handled
    #[test]
    fn multiple_spaces_between_names() {
        let input = "john  McClane"; // two spaces
        let output = name_shuffler(input);
        let expected = "McClane  john"; // two spaces preserved
        assert_eq!(output, expected);
    }

    #[test]
    fn leading_and_trailing_spaces() {
        let input = "  john  McClane  "; // leading and trailing, plus middle double spaces
        let output = name_shuffler(input);
        let expected = "  McClane  john  ";
        assert_eq!(output, expected);
    }

    #[test]
    fn tabs_and_mixed_whitespace() {
        let input = "john\t \tMcClane"; // tab, space, tab between
        let output = name_shuffler(input);
        let expected = "McClane\t \tjohn";
        assert_eq!(output, expected);
    }

    #[test]
    fn single_token_is_unchanged() {
        let input = "Madonna";
        let output = name_shuffler(input);
        let expected = "Madonna";
        assert_eq!(output, expected);
    }

    #[test]
    fn whitespace_only_is_unchanged() {
        let input = "   \t  ";
        let output = name_shuffler(input);
        let expected = "   \t  ";
        assert_eq!(output, expected);
    }

    #[test]
    fn unicode_names() {
        let input = "José Álvarez";
        let output = name_shuffler(input);
        let expected = "Álvarez José";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_randomized() {
        let first_names=vec!["Augustus","Tobias","Vernon","Ryan","Bob","Kareem","Miguel","Cyril","Chris","Simon","Tim"];
        let last_names=vec!["Hill","Beecher","Schillinger","O'Reily","Rebadow","Said","Alvarez","O'Reily","Keller","Adebisi","McManus"];

        let mut rng = rand::rng();
        for _ in 1..40 {
            let first_name = first_names[rng.random_range(0..first_names.len())];
            let last_name = last_names[rng.random_range(0..first_names.len())];
            assert_eq!(name_shuffler(&format!("{} {}", first_name, last_name)),
                       format!("{} {}", last_name, first_name));
        }
    }
}
