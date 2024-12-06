pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod guess_tests {
    use crate::guess::Guess;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn less_than_1() {
        Guess::new(-1);
    }
}