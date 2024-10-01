fn main() {
    let message = "here is a message";
    let apples = 10;
    println!("Hello, world! {}", message);
    println!("I have {} apples", apples);
    let more_apples = apples + 5;
    println!("I now have {} apples", more_apples);
    // some arithmetic
    println!("Let's do some arithmetic");
    let answer = 6 / 2+ 4*3;
    println!("Answer is {}. Following standard Math operations(BODMAS)", answer);
    let another_answer = ((6 / 2) + 4) * 3;
    println!("Another Answer is {}. Following standard Math operations (BODMAS) with brackets", another_answer);
}
