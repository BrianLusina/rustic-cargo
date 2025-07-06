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

    for i in 0..n {
        if visited[i] {
            continue;
        }

        let direction = nums[i].signum();
        let mut slow = i;
        let mut fast = i;

        loop {
            // Move slow pointer
            slow = ((slow as i32 + nums[slow]) % n as i32) as usize;
            if slow < 0 {
                slow += n as i32; // Handle negative indices
            }
            let slow_usize = slow as usize;
            if nums[slow_usize] * direction <= 0 || visited[slow_usize] {
                break; // Invalid direction or already visited
            }

            // Move fast pointer twice
            fast = ((fast as i32 + nums[fast]) % n as i32) as usize;
            if fast < 0 {
                fast += n as i32; // Handle negative indices
            }
            let fast_usize = fast as usize;
            if nums[fast_usize] * direction <= 0 || visited[fast_usize] {
                break; // Invalid direction or already visited
            }
            fast = ((fast as i32 + nums[fast_usize]) % n as i32);
            if fast < 0 {
                fast += n as i32; // Handle negative indices
            }
            let fast_usize = fast as usize;
            if nums[fast_usize] * direction <= 0 || visited[fast_usize] {
                break; // Invalid direction or already visited
            }

            // If slow and fast meet, a cycle is detected
            if slow == fast {
                // Check if the cycle length is greater than 1
                let next_slow = ((slow as i32 + nums[slow_usize]) % n as i32;
                if next_slow < 0 {
                    next_slow += n as i32; // Handle negative indices
                }
                if slow == next_slow {
                    break; // Cycle length is 1, invalid
                }
                return true;
            }
        }

        while direction * nums[slow] > 0 && !visited[slow] {
            visited[slow] = true;
            slow = (slow as i32 + nums[slow]) as usize % n;
            fast = (fast as i32 + nums[fast]) as usize % n;
            if slow == fast {
                let next_slow = (slow as i32 + nums[slow]) as usize % n;
                if slow != next_slow {
                    return true;
                }
                break;
            }
            fast = (fast as i32 + nums[fast]) as usize % n;
        }
    }

    false
}

fn main() {
    let nums1 = vec![2, -1, 1, 2, 2];
    println!("{}", circular_array_loop(nums1)); // Output: true

    let nums2 = vec![-1, 2];
    println!("{}", circular_array_loop(nums2)); // Output: false
}