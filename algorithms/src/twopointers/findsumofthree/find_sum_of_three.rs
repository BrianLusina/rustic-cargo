pub fn find_sum_of_three(numbers: &mut [i32], target: i32) -> bool {
    numbers.sort();

    for idx in 0..numbers.len() - 2 {
        let mut left = idx + 1;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[idx] + numbers[left] + numbers[right];
            if sum == target {
                return true;
            } else if sum > target {
                right -= 1
            } else {
                left += 1
            }
        }
    }

    false
}
