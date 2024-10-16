use std::any::Any;

struct Banana {
    count: i32,
    price: f64,
}

struct Apple {
    count: i32,
    price: f64,
}

struct Store {
    apples: Apple,
    banana: Banana,
}

fn count_apples(apple: Apple) -> () {
    println!("I have {} apples at {} price each", apple.count, apple.count)
}

fn count_bananas(banana: Banana) -> () {
    println!("I have {} bananas at {} price each", banana.count, banana.count)
}

fn store() {
    let apple = Apple {
        count: 10,
        price: 15.5,
    };

    let banana = Banana {
        count: 20,
        price: 25.0
    };

    count_apples(apple);
    count_bananas(banana);
}