use super::primes::primes;

/* nth returns the nth prime number*/
pub fn nth(n: u32) -> u32 {
    primes()
        .nth(n as usize)
        .expect("Prime iterator should be infinite")
}
