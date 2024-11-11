use crate::iterator::iterator::CustomIterator;

// Custom Range struct that counts from 1 to 10
struct OneToTen {
    next_val: u32,
}

impl OneToTen {
    fn new() -> Self {
        OneToTen { next_val: 1 }
    }
}

impl CustomIterator for OneToTen {
    // creating a stream of u32 values
    type Item = u32;

    // Could also be -> Option<u32>
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_val > 10 {
            None
        } else {
            // capture current value
            let result = Some(self.next_val);
            // increase next value by 1
            self.next_val += 1;

            //return previously capture value
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iterator::range::range::OneToTen;

    #[test]
    fn test_one_to_ten() {
        let mut total = 0;
        OneToTen::new();

        assert_eq!(total, 55);
    }
}
