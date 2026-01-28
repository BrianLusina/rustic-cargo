pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut max_consecutive_ones = 0;
    let mut current_consecutive_ones = 0;

    for num in nums {
        if num == 1 {
            current_consecutive_ones += 1;
            max_consecutive_ones = max_consecutive_ones.max(current_consecutive_ones);
        } else {
            current_consecutive_ones = 0;
        }
    }

    max_consecutive_ones
}

#[cfg(test)]
mod find_max_consecutive_ones_tests {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }
}
