// Finds the length of the shortest word
fn find_short(s: &str) -> u32 {
    s.split_whitespace().map(|w| w.len()).min().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::find_short;

    fn dotest(s: &str, expected: u32) {
        let actual = find_short(s);
        assert_eq!(
            actual, expected,
            "With s = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("bitcoin take over the world maybe who knows perhaps", 3);
        dotest(
            "turns out random test cases are easier than writing out basic ones",
            3,
        );
        dotest("lets talk about javascript the best language", 3);
        dotest("i want to travel the world writing code one day", 1);
        dotest("Lets all go on holiday somewhere very cold", 2);
        dotest("Let's travel abroad shall we", 2);
    }
}
