use std::collections::HashMap;

// Checks if a given passed in phrase or sequence of characters in any type is a palindrome, i.e.
// reads the same forwards as it does backwards.
//
// This does this in linear time using constant extra space
//
// Time: O(n) where n is the length of the passed in phrase
// Space: O(1) as no extra space is used as the iteration happens on the provided input
//
// This is achieved by using two pointers, one at the left or the beginning of the string and the
// other at the end or
// the end of the string. A while loop is used to iterate through both ends of the string as long
// as the left pointer is less than the right pointer.
//
// There are two while loops in the parent while loop:
// The first checks that the character at the position of the left pointer is not alphanumeric and
// that it is less than the right pointer and if this condition is met, the left pointer is advanced
// forward(1 is added to it).
//
// Similar case to the second while loop, which checks for this condition in the reverse. If the
// character at the right pointer is not an alphanumeric character and the right pointer is greater
// than the left pointer, then it is decremented by 1, moving it backwards.
//
// The palindrome property is checked by comparing the current lower case version of the character
// at the position of
// the left pointer and the current lower case version of the character at the position of the
// right pointer and if
// they do not match, False is returned immediately. If they are equal, the left pointer and the
// right pointer are
// incremented and decremented respectively to proceed to the next set of characters.
//
// If the condition of this match stays false for all possible iterations of the while loop, True
// is returned at the
// end indicating that the phrase is indeed a palindrome.
pub fn is_palindrome(s: &str) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        while left < right && !s.chars().nth(left).unwrap().is_alphanumeric() {
            left += 1
        }

        while left < right && !s.chars().nth(right).unwrap().is_alphanumeric() {
            right -= 1
        }

        if s.chars().nth(left).unwrap().to_ascii_lowercase()
            != s.chars().nth(right).unwrap().to_ascii_lowercase()
        {
            return false;
        }

        left += 1;
        right -= 1;
    }
    true
}

pub fn is_palindrome_permutation(s: &str) -> bool {
    let normalized = s.replace(" ", "").to_lowercase();

    let mut char_count: HashMap<char, i32> = HashMap::new();

    for c in normalized.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }

    let mut odd_count = 0;

    for (_, count) in char_count {
        let is_even = count % 2 != 0;
        if is_even && odd_count == 0 {
            odd_count += 1;
        } else if is_even && odd_count != 0 {
            return false;
        }
    }

    true
}