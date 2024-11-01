use crate::job;
use job::Job;
use job::Job::{Banker, Farmer, Scientist, Teacher};
use crate::job::{IsTeacher, Subject};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Person {
    age: i32,
    name: String,
    job: Job,
}

impl Person {
    fn new(name: String, age: i32, job: Job) -> Self {
        Person { name, age, job }
    }

    fn say_info(&self) {
        println!("Name: {}, age: {}, job:{:?}", self.name, self.age, self.job)
    }

    fn same_job(&self, other: &Person) -> bool {
        self.job == other.job
    }

    fn greeting(&self, other: &Person) -> String {
        // using pattern matching on the Enum job
        match other.job {
            Teacher(Subject::Art) => {
                format!(
                    "Hello there art teacher, I am {}, I can see you are a {:?} and I am a {:?}",
                    self.name, other.job, self.job
                )
            }
            Teacher(Subject::Math) => {
                format!(
                    "Hello there math teacher, I am {} I can see you are a {:?} and I am a {:?}",
                    self.name, other.job, self.job
                )
            }

            Scientist(_) => {
                format!(
                    "Hello there scientist, I am {} I can see you are a {:?} and I am a {:?}",
                    self.name, other.job, self.job
                )
            }
            Farmer => {
                format!(
                    "Hello there farmer, I am {} I can see you are a {:?} and I am a {:?}",
                    self.name, other.job, self.job
                )
            }
            Banker => {
                format!(
                    "Hello there banker, I am {} I can see you are a {:?} and I am a {:?}",
                    self.name, other.job, self.job
                )
            }
        }
    }
}

impl IsTeacher for Person {
    fn is_teacher(&self) -> bool {
        match self.job {
            Scientist(_) => false,
            Teacher(_) => true,
            Farmer => false,
            Banker => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::job::Job;
    use crate::person::Person;

    #[test]
    fn equality() {
        let alice = Person::new(String::from("Alice"), 30, Job::Farmer);

        let also_alice = alice.clone();
        assert_eq!(alice, also_alice);
        assert!(alice >= also_alice);
        assert!(alice <= also_alice);

        let bob = Person::new(String::from("Bob"), 25, Job::Banker);

        assert_ne!(alice, bob);
    }

    #[test]
    fn same_job() {
        let alice = Person::new(String::from("Alice"), 30, Job::Farmer);
        let bob = Person::new(String::from("Bob"), 25, Job::Farmer);
        assert_eq!(alice.same_job(&bob), true);

        let john = Person::new(String::from("John"), 30, Job::Banker);
        assert_eq!(john.same_job(&alice), false);
    }
}
