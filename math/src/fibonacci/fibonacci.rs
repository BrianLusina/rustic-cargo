struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 && self.next == 1 {
            let new = self.current + self.next;
            self.current = self.next;
            self.next = new;
            return Some(1);
        }

        let new = self.current + self.next;
        self.current = self.next;
        self.next = new;
        Some(self.current)
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
    }
}
