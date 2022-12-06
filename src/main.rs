use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, Rust!");
    println!("#######Guessing Game Start!#######");
    guess_game(false);
    println!("#######Testing Variables Start!#######");
    variables();
}

fn guess_game(is_playing: bool) {
    if is_playing {
        println!("Guess the number!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        // println!("The secret number is {secret_number}");
        println!("Type quit to exit the Game!");

        loop {
            println!("Please enter a number!");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to readl line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    if guess.trim() == "quit" {
                        break;
                    } else {
                        continue;
                    };
                }
            };

            println!("You guessed: {guess}");
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too Small!"),
                Ordering::Greater => println!("Too Big!"),
                Ordering::Equal => {
                    println!("You Win!");
                    break;
                }
            }
        }

        println!("Thanks for playing!");
    }
}

fn variables() {
    // All variables are immutable by default. You can make them mutable
    // by adding "mut"
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Variable shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("Inside - The value of y is: {y}");
    }

    println!("Outside - The value of y is: {y}")
}
