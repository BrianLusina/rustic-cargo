#[derive(Copy, Clone, Debug)]
struct Kmh {
    value: u32
}

impl Kmh {
    fn new(value: u32) -> Kmh {
        Kmh {value}
    }
}

#[derive(Copy, Clone, Debug)]
struct Mph {
    value: u32
}

impl Mph {
    fn new(value: u32) -> Mph {
        Mph {value}
    }
}

#[derive(Copy, Clone, Debug)]
struct Km {
    value: u32
}

#[derive(Copy, Clone, Debug)]
struct Miles {
    value: u32
}

trait InXTime {
    type Distance;
    fn in_x_time(&self, x: u32) -> Self::Distance;
}

impl InXTime for Kmh {
    type Distance = Km;
    fn in_x_time(&self, x: u32) -> Km {
        Km { value: self.value * x }
    }
}

impl InXTime for Mph {
    type Distance = Miles;
    fn in_x_time(&self, x: u32) -> Miles {
        Miles { value: self.value * x }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::speed::{InXTime, Kmh, Mph};

    #[test]
    fn kmh_distance_calculation() {
        let speed = Kmh { value: 90 };
        let distance = speed.in_x_time(3);
        let expectedDistance = 270;
        assert_eq!(distance.value, expectedDistance);
    }

    #[test]
    fn mph_distance_calculation() {
        let speed = Mph { value: 90 };
        let distance = speed.in_x_time(3);
        let expectedMiles = 270;
        assert_eq!(distance.value, expectedMiles);
    }
}