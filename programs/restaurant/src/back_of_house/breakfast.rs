pub struct Breakfast {
    // toast will be made available(public) to other modules
    pub toast: String,
    // seasonal_fruit will remain a private field to this struct
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
