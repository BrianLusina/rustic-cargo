#[cfg(test)]
mod prime_tests {
    use math::prime::primes::primes;

    #[test]
    fn generates_first_ten_primes() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let got: Vec<u32> = primes().take(10).collect();
        assert_eq!(got, expected);
    }

    #[test]
    fn sequence_is_increasing_for_first_100() {
        let mut iter = primes().take(100);
        let mut prev = iter.next().expect("should yield first prime");
        for p in iter {
            assert!(p > prev, "sequence not increasing: {} then {}", prev, p);
            prev = p;
        }
    }

    #[test]
    fn all_primes_after_two_are_odd_for_first_200() {
        let mut iter = primes().take(200);
        let first = iter.next().unwrap();
        assert_eq!(first, 2);
        for p in iter {
            assert!(p % 2 == 1, "found even non-two prime {}", p);
        }
    }
}
