/**
Function that determines if a plural is needed or not. Takes a number and returns True if a plural
should be used with that number or false if not. This is useful when printing out a string such as
5 minutes, 14 apples, or 1 sun
**/
pub fn plural(n: f64) -> bool {
    n != 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        assert_eq!(plural(0.0), true);
        assert_eq!(plural(0.5), true);
        assert_eq!(plural(1.0), false);
        assert_eq!(plural(100.0), true);
    }
}
