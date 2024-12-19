fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    strings::utils::longest::longest(x, y)
}

#[cfg(test)]
mod longest_tests {
    use crate::lifetimes::longest::longest_with_an_announcement;

    #[test]
    fn it_works() {
        let x = String::from("hello");
        let y = String::from("world");
        let result = longest_with_an_announcement(&x, &y, "hello");
        assert_eq!(result, "world");
    }
}
