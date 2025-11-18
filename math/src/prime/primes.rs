pub struct Primes {
    primes: Vec<u32>,
    next_candidate: u32,
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Handle the first prime explicitly
        if self.primes.is_empty() {
            self.primes.push(2);
            self.next_candidate = 3; // start checking odd numbers next
            return Some(2);
        }

        // Search for the next prime by testing odd candidates
        loop {
            let n = self.next_candidate;
            // Stop once we cannot advance by 2 without overflowing
            if self.next_candidate > u32::MAX - 2 {
                return None;
            }
            self.next_candidate += 2;

            let mut is_prime = true;
            for &p in &self.primes {
                // Stop trial division at sqrt(n)
                if (p as u64) * (p as u64) > n as u64 {
                    break;
                }
                if n.is_multiple_of(p) {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                self.primes.push(n);
                return Some(n);
            }
            // otherwise, continue with next odd candidate
        }
    }
}

// Create an (infinite) generator of prime numbers
pub fn primes() -> Primes {
    Primes {
        primes: Vec::new(),
        next_candidate: 2,
    }
}
