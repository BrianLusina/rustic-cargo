use std::collections::HashMap;

// demo of updating the values of keys in a hash map
fn count_words() -> () {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{map:?}")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::maps::map_basics::count_words;

    #[test]
    fn test_basics() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("{team_name} has a score of {score}");

        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        scores.insert(String::from("Red"), 20);
        scores.insert(String::from("Blue"), 20);
        println!("{scores:?}");

        scores.entry(String::from("Yellow")).or_insert(70);
        scores.entry(String::from("Blue")).or_insert(100);
        scores.entry(String::from("Magenta")).or_insert(500);
        println!("{scores:?}");

        count_words();
    }
}

