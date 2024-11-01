pub trait IsTeacher {
    fn is_teacher(&self) -> bool;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Job {
    Scientist(Field),
    Teacher(Subject),
    Farmer,
    Banker
}

impl IsTeacher for Job {
    fn is_teacher(&self) -> bool {
        match self {
            Job::Teacher(_) => true,
            Job::Scientist(_) => false,
            Job::Farmer => false,
            Job::Banker => false,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Subject {
    Art,
    Math
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Field {
    Physics,
    Biology,
}

enum CongressPerson {
    Representative(State, District),
    Senator(State, Seniority),
}

enum State {
    Alabama,
    Alaska,
    // this could take a while
    Wyoming,
}

struct District {
    number: u32,
}

enum Seniority {
    Junior,
    Senior,
}