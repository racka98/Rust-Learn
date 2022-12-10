#![allow(unused)]

use core::fmt::Debug;
use std::fmt::Display;

pub fn actions() {
    // Reusable code with functions
    let number_list = vec![34, 50, 25, 100, 65];
    println!(
        "The largest number is {}",
        find_largest_number(&number_list)
    );
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!(
        "The largest number is {}",
        find_largest_number(&number_list)
    );

    // Generic data types
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));

    let integer = Point { x: 4, y: -1 };
    let float = Point { x: 3.0, y: -1.2 };
    let float_and_integer = Point { x: 3.0, y: -1 };

    // Using traits
    let tweet = Tweet {
        username: String::from("rackadev"),
        content: String::from("Rust is a pretty cool language! Almost as cool as Kotlin 💖"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet -> {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Google Releases new programming language"),
        location: String::from("Seatle"),
        author: String::from("codelover360"),
        content: String::from(
            "Google releases a new programming language called Carbon which aims to be a C++ successor!"
        ),
    };

    notify(&article);
    println!("{}", article.content);

}

// Defines a lifetime for the values
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn find_largest_number(numbers: &Vec<i32>) -> i32 {
    let mut largest = numbers[0];
    for number in numbers {
        if *number > largest {
            largest = *number;
        }
    }
    largest
}

// Find largest with generics
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generics in structs & enums
struct Point<T, U> {
    x: T,
    y: U,
}

enum Power<V, T> {
    Voltage(V),
    Current(T),
}

// Traits
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author()) // You also have a default impl
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("Journalist: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Trait bound syntax. Which is the actual syntax for `impl Trait`
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound with multiple traits
pub fn notify_display<T: Summary + Display>(item: &T) {
    println!("Item: {}", item);
    println!("Breaking news! {}", item.summarize());
}

// Trait bound with `where` clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}

// Returning types that impl traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
