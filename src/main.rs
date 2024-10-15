fn main() {
    let message = "here is a message";
    let apples = 10;
    println!("Hello, world! {}", message);
    println!("I have {} apples", apples);
    let more_apples = apples + 5;
    println!("I now have {} apples", more_apples);
    // some arithmetic
    println!("Let's do some arithmetic");
    let answer = 6 / 2 + 4 * 3;
    println!(
        "Answer is {}. Following standard Math operations(BODMAS)",
        answer
    );
    let another_answer = ((6 / 2) + 4) * 3;
    println!(
        "Another Answer is {}. Following standard Math operations (BODMAS) with brackets",
        another_answer
    );

    let number = if apples < 10 { apples } else { apples };
    println!("Number: {}", number);

    // tuple
    let tup = (90, "ser", 'a');
    // destructuring
    let (x, y, z) = tup;
    // index access
    let ninety = tup.0;
    let set = tup.1;
    let a = tup.2;

    // arrays
    let coll: [i32; 4] = [1, 2, 3, 4];
    let coll2 = [5; 5]; // all positions in the array will be 5, a concise way of writing an array

    // element access of array with indices
    let first = coll2[0];
    let second = coll2[3];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let mut counter = 0;
    while counter < 4 {
        println!("Yo!, I am in a while loop!");
        counter += 1;
    }

    fibo
}
