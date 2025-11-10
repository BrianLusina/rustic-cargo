mod enums;
mod errors;
mod functions;
mod generics;
mod lifetimes;
mod maps;
mod ownership;
mod sets;
mod structs;
mod traits;

fn main() {
    functions::say_apples(10);
    functions::double(12);
    functions::eat(10, "apples");

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");
}
