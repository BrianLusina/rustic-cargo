mod guess;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use crate::guess::Guess;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut user_input_guess = String::new();

        io::stdin()
            .read_line(&mut user_input_guess)
            .expect("Failed to read line");

        let input_guess_num: i32 = match user_input_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(input_guess_num);

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
