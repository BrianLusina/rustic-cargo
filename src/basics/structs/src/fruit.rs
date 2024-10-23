use std::any::Any;

struct Banana {
    count: i32,
    price: f64,
}

impl Banana {
    fn new(count: i32, price: f64) -> Banana {
        Banana {
            count: count,
            price: price,
        };
    }

    fn increase_by(&mut self, multiplier: i32) -> () {
        self.price *= multiplier;
    }
}

struct Apple {
    count: i32,
    price: f64,
}

impl Apple {
    fn new(count: i32, price: f64) -> Apple {
        Apple {
            count: count,
            price: price,
        };
    }

    fn increase_by(&mut self, multiplier: i32) -> () {
        self.price *= multiplier;
    }

    fn print(&self) {
        println!("I have {} apples at price {}", self.count, self.price);
    }
}

struct Store {
    apple: Apple,
    banana: Banana,
}

impl Store {
    fn new(apple: Apple, banana: Banana) -> Store {
        Store {
            banana: banana,
            apples: apple,
        };
    }

    fn count_apples(&self) -> () {
        println!(
            "I have {} apples at {} price each",
            self.apples.count, self.apples.price
        )
    }

    fn count_bananas(&self) -> () {
        println!(
            "I have {} bananas at {} price each",
            self.banana.count, self.banana.price
        )
    }
}

fn store() {
    let apple = Apple {
        count: 10,
        price: 15.5,
    };

    let banana = Banana {
        count: 20,
        price: 25.0,
    };

    let store = Store {
        banana: banana,
        apples: apple,
    };

    store.count_apples(apple);
    store.count_bananas(banana);
}
