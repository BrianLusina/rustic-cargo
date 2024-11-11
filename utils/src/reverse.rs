// reverse reverses a slice of signed 32-bit integers and returns a vector of signed 32 bit integers
// First becomes last and so on.
fn reverse(numbers: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(numbers.len());
    for i in (0..numbers.len()).rev() {
        result.push(numbers[i]);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::reverse::reverse;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(&[]), vec![]);
        assert_eq!(reverse(&[1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(reverse(&[1, 2, 3, 4]), vec![4, 3, 2, 1]);
        assert_eq!(reverse(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
        assert_eq!(reverse(&[1, 2, 3, 6, 7, 8]), vec![8, 7, 6, 3, 2, 1]);
    }
}
