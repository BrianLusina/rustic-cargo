fn find_maximum(numbers: &[usize]) -> usize {
    let mut max = 0;
    for number in numbers {
        if number > &max {
            max = *number;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::max::find_maximum;

    #[test]
    fn test_find_maximum() {
        assert_eq!(find_maximum(&[4, 2, 3]), 4);
    }
}
