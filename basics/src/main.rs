mod enums;
mod functions;
mod maps;
mod ownership;
mod structs;
mod errors;

fn main() {
    functions::say_apples(10);
    functions::double(12);
    functions::eat(10, "apples");

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");
}
