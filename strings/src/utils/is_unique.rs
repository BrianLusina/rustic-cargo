use std::collections::HashMap;

// Checks that an input string has unique characters and returns true if it does. if not, false is
// returned
pub fn is_unique(input_string: &str) -> bool {
    let mut char_map: HashMap<char, bool> = HashMap::new();

    for char in input_string.chars() {
        if char_map.contains_key(&char) {
            return false;
        } else {
            char_map.insert(char, true);
        }
    }

    true
}
