fn average(scores: &Vec<usize>) -> usize {
    let mut sum = 0;
    for x in scores.iter() {
        sum += x;
    }

    sum / scores.len()
}

#[cfg(test)]
mod tests {
    use crate::average::average;

    #[test]
    fn calculate_average_1() {
        let scores = vec![53, 64, 73, 36, 96, 100, 45, 81, 88, 64];
        let actual = average(&scores);
        let expected: usize = 70;
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_average_2() {
        let scores = vec![68, 29, 87, 34, 89, 95];
        let actual = average(&scores);
        let expected: usize = 67;
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_average_3() {
        let scores = vec![100, 81, 68, 60, 86, 79, 88, 94];
        let actual = average(&scores);
        let expected: usize = 82;
        assert_eq!(expected, actual);
    }
}
