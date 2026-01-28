pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_works() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn capitalize_long_string() {
        let s = "hello world";
        let expected = "Hello world";
        let actual = capitalize(s);
        assert_eq!(actual, expected);
    }
}
