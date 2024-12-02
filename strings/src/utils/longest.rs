// retrieves the longest string between two passed in strings x and y and returns the longest of the
// two arguments.
// Note the usage of the lifetime parameter 'a. Since this function is not clear which of the two
// references passed in will be returned, this is used to specify a lifetime and allow the borrow
// checker on the compiler to either return the concret type x or y. Also, this function does not
// need to take ownership of either x nor y, hence the usage of references with an addition of the
// lifetime parameter.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod longest_tests {
    use crate::utils::longest::longest;

    #[test]
    fn test_longest() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let expected = "abcd";
        let actual = longest(string1.as_str(), string2);
        assert_eq!(expected, actual);
    }
}
