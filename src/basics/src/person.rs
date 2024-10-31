#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Person {
    age: i32,
    name: String,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person {
            name,
            age
        }
    }

    fn say_info(&self) {
        println!("Name: {}, age: {}", self.name, self.age)
    }
}

#[cfg(test)]
mod tests {
    use crate::person::Person;

    #[test]
    fn equality() {
        let alice = Person::new(String::from("Alice"), 30);

        let also_alice = alice.clone();
        assert_eq!(alice, also_alice);
        assert!(alice >= also_alice);
        assert!(alice <= also_alice);

        let bob = Person::new(String::from("Bob"), 25);

        assert_ne!(alice, bob);
    }
}