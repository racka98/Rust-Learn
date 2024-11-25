mod closures;
mod collections;
mod enums;
mod error_handling;
mod generics_traits_lifetimes;
mod fibonacci;
mod ownership;
mod structs;
mod tests;
mod smart_pointers;
mod concurrency;

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, Rust!");
    println!("#######Guessing Game Start!#######");
    guess_game(false);
    println!("#######Testing Variables Start!#######");
    variables();
    println!("#######Data Types Start!#######");
    data_types();
    println!("#######Control Flows Start!#######");
    control_flow();
    println!("##### Fibonacci Func Start!#######");
    fibonacci::fibonacci_num(10);
    ownership::variable_ownership();
    println!("##### Structs Start!#######");
    structs::stuct_test();
    println!("##### Enums Start!#######");
    enums::enums_actions();
    println!("##### Collections Start! #######");
    collections::collections_actions();
    println!("##### Error Handling Start! #######");
    error_handling::error_handling();
    println!("##### Generics, Traits & Lifetimes Start! #######");
    generics_traits_lifetimes::actions();
    println!("##### Closures & FP Start! #######");
    closures::actions();
    println!("##### Smart Pointers Start! #######");
    smart_pointers::smart_pointers();
    println!("##### Concurrency Start! #######");
    concurrency::concurrency();
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
                .expect("Failed to read line");

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

    println!("Outside - The value of y is: {y}");
}

fn data_types() {
    // Scalar Types
    /*
     * Integer data types include 8bit, 16bit, 32bit, 64bit, 128bit and arch (isize)
     */
    let integer = 2;
    let decimal = 90_320;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!(
        "Integer: {integer}, Decimal: {decimal}, Hex: {hex}, Octal: {octal}, 
        Binary: {binary}, Byte: {byte}"
    );

    /*
     * Floating point types include f32 (same as Float in Kotlin) &
     * f64 (same as Double in Kotlin)
     */
    let x = 2.1;
    let y: f32 = 20.1;
    let ans = x + y;
    println!("X value: {x}, Y value: {y}. Ans is: {ans}");

    // Numeric operation are similar to other langs like Kotlin

    // Boolean type
    let t = true;
    let f = false;
    println!("Say {t} or {f}");

    // Character type
    let letter1 = 'a';
    let num1 = '1';
    println!("First letter {letter1}, First number {num1}");

    // Compounded Types
    // Tupples
    let tup: (i32, f64, i8) = (500, 2.4, 1);
    let (first, second, last) = tup;
    println!("Tupple First: {first}, Second: {second} & Last: {last}");

    // Arrays
    let array = [0, 1, 2, 3, 4, 5];
    for num in array {
        println!("Number: {num}");
    }
}

fn control_flow() {
    // Simple if statement
    let num = 30;
    if num >= 20 {
        println!("This num is huge");
    } else if num == 30 {
        println!("Right on the money!")
    } else {
        println!("This number is smol!");
    }

    // Assigning from if statement
    let is_on = true;
    let ac_value = if is_on { 30 } else { 0 };
    println!("Ac temp: {ac_value}C");

    // Returning value from Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("Looping...");
        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {result}");

    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count <= 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
