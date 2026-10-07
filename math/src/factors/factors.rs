pub fn kth_factor(n: i32, k: i32) -> i32 {
    let mut counter = k;
    for i in 1..=n+1 {
        if n % i == 0 {
            counter -= 1;
            if counter == 0 {
                return i;
            }
        }
    }
    -1
}

#[cfg(test)]
mod factor_tests {
    use parameterized::parameterized;
    use parameterized::ide;
    use crate::factors::factors::kth_factor;

    ide!();

    #[parameterized(
        n = {12, 7, 4},
        k = {3, 2, 4},
        expected = {3, 7, -1}
    )]
    fn test_kth_factor(n: i32, k: i32, expected: i32) {
        let actual = kth_factor(n, k);
        assert_eq!(expected, actual);
    }
}