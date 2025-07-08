/// Determines if there is a cycle in the given circular array.
///
/// The function checks for cycles in a circular array using the fast and slow
/// pointer technique. It iterates through each index and uses the values in the
/// array to determine the movement. It validates the direction of movement and
/// checks for a cycle length greater than one.
///
/// # Arguments
///
/// * `nums` - A vector of integers representing the circular array.
///
/// # Returns
///
/// * `bool` - Returns `true` if there is a cycle in the array; otherwise, `false`.
///
/// # Details
///
/// The function utilizes the fast and slow pointer strategy to detect cycles in
/// the array. It ensures that the movement remains within the same direction
/// (either all positive or all negative). If a cycle is found where the fast and
/// slow pointers meet, the function confirms that the cycle length is greater
/// than one before returning `true`.
pub fn circular_array_loop(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut visited = vec![false; n];

    // Helper function to get the next position in the circular array
    let next_position = |current: usize, nums: &[i32]| -> usize {
        let next = (current as i32 + nums[current]) % n as i32;
        if next < 0 {
            (next + n as i32) as usize
        } else {
            next as usize
        }
    };

    for i in 0..n {
        if visited[i] {
            continue;
        }

        let direction = nums[i].signum();
        let mut slow = i;
        let mut fast = i;

        loop {
            // Move slow pointer
            slow = next_position(slow, &nums);
            if nums[slow] * direction <= 0 || visited[slow] {
                break; // Invalid direction or already visited
            }

            // Move fast pointer twice
            fast = next_position(fast, &nums);
            if nums[fast] * direction <= 0 || visited[fast] {
                break; // Invalid direction or already visited
            }

            fast = next_position(fast, &nums);
            if nums[fast] * direction <= 0 || visited[fast] {
                break; // Invalid direction or already visited
            }

            // If slow and fast meet, a cycle is detected
            if slow == fast {
                // Check if the cycle length is greater than 1
                let next_slow = next_position(slow, &nums);
                if slow == next_slow {
                    break; // Cycle length is 1, invalid
                }
                return true;
            }
        }

        // Mark all indices in this failed path as visited
        let mut current = i;
        while direction * nums[current] > 0 && !visited[current] {
            visited[current] = true;
            current = next_position(current, &nums);
        }
    }

    false
}
