use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Student<'a>{
    first_name: &'a str,
    last_name: &'a str,
}

#[derive(Debug)]
struct ScoreInfo<'a>{
    subject: &'a str,
    score: &'a str,
}

type Grades<'a> = HashMap<Student<'a>, ScoreInfo<'a>>;

fn main() {
    let mut students: Grades = HashMap::new();

    let jason = Student {
    first_name: "Jason",
    last_name: "Doe",
    };

    let grade1 = ScoreInfo {
    subject: "Math",
    score: "passing grade",
    };

    students.insert(jason, grade1);

    println!("Jason's score info: {:?}",
             students.get(
                 &Student{first_name: "Jason", last_name: "Doe"}
             ).unwrap()
    );
}
