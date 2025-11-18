#[cfg(test)]
mod is_prime_utils_tests {
    use math::prime::utils::is_prime;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(11));
    }
}

#[cfg(test)]
mod primes_up_to_tests {
    use math::prime::utils::primes_up_to;

    #[test]
    fn test_primes_up_to_max_of_6() {
        let expected = Vec::from([2, 3, 5, 7]);
        let actual = primes_up_to(9);
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod generate_primes_tests {
    use math::prime::utils::generate_primes;

    #[test]
    fn test_primes_up_to_6() {
        let expected = Vec::from([2, 3, 5, 7, 11, 13]);
        let actual = generate_primes(6);
        assert_eq!(expected, actual);
    }
}
