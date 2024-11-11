fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn fibs(count: usize) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::with_capacity(count);
    let mut i: u32 = 0;

    while i < count as u32 {
        let fib_number = fibonacci(i);
        vec.push(fib_number);
        i += 1;
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::fibonacci::fibonacci;

    #[test]
    fn basic() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn zero() {
        assert_eq!(fibonacci(0), 1);
    }

    #[test]
    fn one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn two() {
        assert_eq!(fibonacci(2), 2);
    }
}

#[cfg(test)]
mod fibs_tests {
    use crate::fibonacci::fibs;

    #[test]
    fn basic() {
        assert_eq!(fibs(2), vec![1, 1]);
    }

    #[test]
    fn count_of_five() {
        assert_eq!(fibs(5), vec![1, 1, 2, 3, 5]);
    }

    #[test]
    fn count_of_eight() {
        assert_eq!(fibs(8), vec![1, 1, 2, 3, 5, 8, 13, 21]);
    }
}
