use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess a number!");

    let secret_number: u16 = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line...");

        if guess.trim() == "quit" {
            println!("You quit the game!");
            break;
        }

        println!("You guessed: {guess}");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
