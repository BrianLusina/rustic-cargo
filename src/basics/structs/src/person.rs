struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person {
            name: name,
            age: age
        }
    }

    fn say_info(&self) {
        println!("Name: {}, age: {}", self.name, self.age)
    }
}