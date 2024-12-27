use std::collections::HashMap;
use crate::utils::sort_word::sort_string_unstable;

// checks that two strings are permutations of each other and returns true if they are or false
// otherwise.
//
// Complexity:
// Time: O(nlog(n)) as sorting is used and sorting here is an O(nlog(n)) time complexity
// Space: O(1) as no extra space is used to achieve checking for permutations
pub fn check_permutation_with_sorting(input_str_1: &str, input_str_2: &str) -> bool {
    // if the strings are of different lengths, we return false. strings of different lengths are not
    // permutations of each other
    if input_str_1.len() != input_str_2.len() {
        return false;
    }

    let input_str_1_sorted = sort_string_unstable(&input_str_1.to_lowercase());
    let input_str_2_sorted = sort_string_unstable(&input_str_2.to_lowercase());

    for index in 0..input_str_1_sorted.len() {
        if input_str_1_sorted.as_bytes()[index] != input_str_2_sorted.as_bytes()[index] {
            return false;
        }
    }

    true
}

// checks that two strings are permutations of each other and returns true if they are or false
// otherwise.
//
// Complexity:
// Time: O(n) as it iterates through the strings once
// Space: O(n) as it uses a map to store the frequency of each character in the strings
pub fn check_permutation_with_map(input_str_1: &str, input_str_2: &str) -> bool {
    if input_str_1.len() != input_str_2.len() {
        return false;
    }

    let input_str_1_lower = input_str_1.to_lowercase();
    let input_str_2_lower = input_str_2.to_lowercase();

    let mut char_count_map: HashMap<char, i32> = HashMap::new();

    for c in input_str_1.chars() {
        let count = char_count_map.entry(c).or_insert(0);
        *count += 1;
    }

    for c in input_str_2.chars() {
        let count = char_count_map.entry(c).or_insert(0);
        *count -= 1;
    }

    char_count_map.values().all(|count| count == &0)
}
