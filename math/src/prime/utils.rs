// Function to check if a single number is prime
pub fn is_prime(n: u64) -> bool {
    // Handle the base cases: numbers less than 2 are not prime
    if n < 2 {
        return false;
    }

    // 2 is the only even prime number
    if n == 2 {
        return true;
    }

    // All other even numbers are not prime
    if n % 2 == 0 {
        return false;
    }

    // Check odd divisors up to the square root of n
    // We only need to check up to sqrt(n) because if n has a divisor
    // greater than sqrt(n), it must also have a corresponding divisor
    // less than sqrt(n)
    let limit = (n as f64).sqrt() as u64;

    // Start at 3 and check only odd numbers (step by 2)
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

// Function to generate all primes up to a maximum value
pub fn primes_up_to(max: u64) -> Vec<u64> {
    let mut primes = Vec::new();

    for candidate in 2..=max {
        if is_prime(candidate) {
            primes.push(candidate);
        }
    }

    primes
}

// Function to generate the first n prime numbers
pub fn generate_primes(count: usize) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut candidate = 2;

    // Keep checking numbers until we have enough primes
    while primes.len() < count {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes
}
