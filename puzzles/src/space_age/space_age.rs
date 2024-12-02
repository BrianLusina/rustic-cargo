#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

const EARTH_ORBITAL: f64 = 31_557_600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

impl From<f64> for Duration {
    fn from(s: f64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn orbital_duration() -> Duration;

    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::orbital_duration().seconds
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_duration() -> Duration {
        Duration::from(0.2408467 * EARTH_ORBITAL)
    }
}

impl Planet for Venus {
    fn orbital_duration() -> Duration {
        Duration::from(0.61519726 * EARTH_ORBITAL)
    }
}

impl Planet for Earth {
    fn orbital_duration() -> Duration {
        Duration::from(EARTH_ORBITAL)
    }
}

impl Planet for Mars {
    fn orbital_duration() -> Duration {
        Duration::from(1.8808158 * EARTH_ORBITAL)
    }
}

impl Planet for Jupiter {
    fn orbital_duration() -> Duration {
        Duration::from(11.862615 * EARTH_ORBITAL)
    }
}

impl Planet for Saturn {
    fn orbital_duration() -> Duration {
        Duration::from(29.447498 * EARTH_ORBITAL)
    }
}

impl Planet for Uranus {
    fn orbital_duration() -> Duration {
        Duration::from(84.016846 * EARTH_ORBITAL)
    }
}

impl Planet for Neptune {
    fn orbital_duration() -> Duration {
        Duration::from(164.79132 * EARTH_ORBITAL)
    }
}

#[cfg(test)]
mod space_age_tests {
    use crate::space_age;
    use space_age::space_age::*;

    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();
        let delta: f64 = 0.01;
        if diff > delta {
            panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
        }
    }

    #[test]
    fn age_on_earth() {
        let seconds = 1_000_000_000;
        let duration = Duration::from(seconds);
        let output = Earth::years_during(&duration);
        let expected = 31.69;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_mercury() {
        let seconds = 2_134_835_688;
        let duration = Duration::from(seconds);
        let output = Mercury::years_during(&duration);
        let expected = 280.88;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_venus() {
        let seconds = 189_839_836;
        let duration = Duration::from(seconds);
        let output = Venus::years_during(&duration);
        let expected = 9.78;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_mars() {
        let seconds = 2_129_871_239;
        let duration = Duration::from(seconds);
        let output = Mars::years_during(&duration);
        let expected = 35.88;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_jupiter() {
        let seconds = 901_876_382;
        let duration = Duration::from(seconds);
        let output = Jupiter::years_during(&duration);
        let expected = 2.41;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_saturn() {
        let seconds = 2_000_000_000;
        let duration = Duration::from(seconds);
        let output = Saturn::years_during(&duration);
        let expected = 2.15;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_uranus() {
        let seconds = 1_210_123_456;
        let duration = Duration::from(seconds);
        let output = Uranus::years_during(&duration);
        let expected = 0.46;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_neptune() {
        let seconds = 1_821_023_456;
        let duration = Duration::from(seconds);
        let output = Neptune::years_during(&duration);
        let expected = 0.35;
        assert_in_delta(expected, output);
    }
}
