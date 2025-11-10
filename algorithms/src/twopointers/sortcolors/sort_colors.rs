///
/// Sorts a list of 0s, 1s, and 2s in place
///
/// This algorithm uses three pointers: low, mid, and high. The low pointer is used to track the
/// position where the next 0 should be placed, the mid-pointer is used to scan the list, and the
/// high pointer is used to track the position where the next 2 should be placed.
///
/// The algorithm works by iterating through the list with the mid-pointer. If it encounters a 0,
/// it swaps it with the element at the low index and increments both low and mid. If it encounters
/// a 1, it simply increments mid. If it encounters a 2, it swaps it with the element at the high
/// index and decrements high (without incrementing mid, because the swapped element could be a 0
/// or a 1).
///
/// This algorithm has a time complexity of O(n) and a space complexity of O(1), making it efficient
/// for sorting lists of 0s, 1s, and 2s.
/// # Arguments
///
/// * `colors`:
///
/// returns: &mut Vec<i32, Global>
///
/// # Examples
///
/// ```
///
/// ```
pub fn sort_colors(colors: &mut [i32]) -> &mut [i32] {
    let mut low = 0;
    let mut mid = 0;
    let mut high = colors.len() - 1;

    while mid <= high {
        if colors[mid] == 0 {
            let m = colors[mid];
            colors[mid] = colors[low];
            colors[low] = m;
            low += 1;
            mid += 1;
        } else if colors[mid] == 1 {
            mid += 1;
        } else {
            // this prevents the high not being decremented below 0, preventing underflow errors
            // while keeping the original while loop condition as is.
            if high == 0 {
                break;
            }
            let m = colors[mid];
            colors[mid] = colors[high];
            colors[high] = m;
            high -= 1;
        }
    }

    colors
}
