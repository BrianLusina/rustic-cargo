/// Returns a new array with the squares of the elements from the input array,
/// sorted in ascending order.
///
/// # Arguments
///
/// * `nums` - A vector of integers sorted in ascending order
///
/// # Returns
///
/// * A vector of integers containing the squares of the input elements, sorted in ascending order
///
/// # Time Complexity
///
/// * O(n) where n is the length of the input array
///
/// # Space Complexity
///
/// * O(n) for the result array
///
/// # Example
///
/// ```
/// use algorithms::arrays::sortedsquaredarray::sorted_squared_array::sorted_squares;
///
/// let nums = vec![-4, -1, 0, 3, 10];
/// let result = sorted_squares(nums);
/// assert_eq!(result, vec![0, 1, 9, 16, 100]);
/// ```
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    // Handle empty array case
    if n == 0 {
        return vec![];
    }

    // For single element, just return its square
    if n == 1 {
        return vec![nums[0] * nums[0]];
    }

    let mut result = vec![0; n];
    let mut left: usize = 0;
    let mut right: usize = n - 1;
    let mut current_index: usize = n - 1;

    // Fill the result array from right to left (largest to smallest)
    // This approach works because the largest squared value will either be at the beginning
    // of the array (if it's a negative number with large absolute value) or at the end of the array
    while left < right {
        let left_square = nums[left] * nums[left];
        let right_square = nums[right] * nums[right];

        if left_square > right_square {
            result[current_index] = left_square;
            left += 1;
        } else {
            result[current_index] = right_square;
            right -= 1;
        }
        current_index -= 1;
    }

    // Handle the case when left == right (last element)
    result[current_index] = nums[left] * nums[left];

    result
}
