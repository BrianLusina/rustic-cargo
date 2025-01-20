use std::collections::HashMap;

// Checks that an input string has unique characters and returns true if it does. if not, false is
// returned
pub fn is_unique(input_string: &str) -> bool {
    let mut charMap: HashMap<char, bool> = HashMap::new();

    for char in input_string.chars() {
        if charMap.contains_key(&char) {
            return false;
        } else {
            charMap.insert(char, true);
        }
    }

    true
}
