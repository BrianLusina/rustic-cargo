pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut running_sum = vec![nums[0]];
    for i in 1..nums.len() {
        running_sum.push(running_sum[i - 1] + nums[i]);
    }
    running_sum
}

#[cfg(test)]
mod running_sum_tests {
    use parameterized::{parameterized, ide};

    ide!();

    use super::*;

    #[parameterized(
        nums = {
            vec![1, 2, 3, 4],
            vec![1, 1, 1, 1, 1],
            vec![3, 1, 2, 10, 1]
        },
        expected = {
            vec![1, 3, 6, 10],
            vec![1, 2, 3, 4, 5],
            vec![3, 4, 6, 16, 17]
        }
    )]
    fn test_running_sum(nums: Vec<i32>, expected: Vec<i32>) {
        let actual = running_sum(nums);
        assert_eq!(actual, expected);
    }
}