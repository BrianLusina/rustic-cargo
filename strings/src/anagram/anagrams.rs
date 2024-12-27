use crate::utils::sort_word::sort_string_unstable;
use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_case_word = word.to_lowercase();
    let sorted = sort_string_unstable(&lower_case_word);

    possible_anagrams
        .iter()
        .filter(|possible_anagram| {
            let lower = possible_anagram.to_lowercase();
            lower_case_word != lower && sorted == sort_string_unstable(&lower)
        })
        .cloned()
        .collect()
}

pub fn is_anagram(word1: &str, word2: &str) -> bool {
    // first normalize the strings by removing white spaces which might result in uneven lengths if
    // s1 and s2 are anagrams of each other
    let word1 = word1.replace(" ", "").to_lowercase();
    let word2 = word2.replace(" ", "").to_lowercase();

    if word1.len() != word2.len() {
        return false;
    }

    // This map is used to keep track of the character count in the strings to check if the strings are anagrams
    // of each other, the character count must be equal in both strings. This enables the algorithm to keep track of this
    // count.
    let mut char_count: HashMap<char, i32> = HashMap::new();

    // Loop through each character in the first string to count the number of characters and store them in the map
    // this is linear, so, O(n) where n is the number of characters in the string as the loop has to iterate over each
    // character
    for c in word1.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }

    // Loops through each character in the second string checking for the existence of that character in the already
    // populated map. If a character, exists, the count is decremented, if not, the count is incremented. This
    // will be used to show the discrepancy in character count between the two strings
    for c in word2.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count -= 1;
    }

    // Finally, check each key in the map. If a given key's count is not equal to 0, then the algorithm exits
    // early as it's not possible to have a character count of more than 0 for strings that are anagrams, since the above
    // loop should have reduced the character count to 0. This shows a discrepancy, meaning there is an extra character
    // in a string that is not in another string
    for (_, count) in char_count {
        if count != 0 {
            return false;
        }
    }

    true
}
