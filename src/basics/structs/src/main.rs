mod functions;
mod fruit;
mod ownership;

fn main() {
    functions::say_apples(10);
    functions::double(12);
    functions::eat(10, "apples");

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");
}