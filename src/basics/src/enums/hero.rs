enum Hero {
    Goku,
    Vegeta,
    Superman,
}

impl Hero {
    fn power_level(&self) -> Option<u32> {
        match self {
            Hero::Goku => Some(100),
            Hero::Vegeta => Some(200),
            Hero::Superman => None
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Hero::Goku => "Goku",
            Hero::Vegeta => "Vegeta",
            Hero::Superman => "Superman",
        }
    }

    fn print_power(&self) {
        match self.power_level() {
            None => {
                println!("I don't know {}'s power level", self.name());
            }
            Some(level) => {
                println!("{}'s power level is {}", self.name(), level);
            }
        }
    }
}