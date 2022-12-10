#![allow(unused)]

use std::collections::HashMap;
pub fn collections_actions() {

    // Vectors 
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];
    v.push(7);
    v.push(9);
    v.push(6);
    v.push(4);
    v1.append(&mut v.clone()); // Just move the clone and leave v intact

    v.sort();

    for i in &v {
        println!("V value is {}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("Update V value is {}", i);
    }

    let third = &v1[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("No third element"),
    };

    // Using enums to hold different values in a Vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Int(val) => println!("Cell Value: {val}"),
            SpreadsheetCell::Text(val) => println!("Cell Value: {val}"),
            SpreadsheetCell::Float(val) => println!("Cell Value: {val}"),
        }
    }

    // Strings
    let mut s = String::new();
    s.push_str("Hello");
    s.push('.');
    println!("Greeting: {}", s);

    let data = "initial contents";
    let s1 = data.to_string();
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let combined = s + &hello;

    let hello = "Здравствуйте";
    let my_chars = hello.chars();
    let bytes = hello.as_bytes();
    let answer = &hello[..2];
    println!("First char = {}", answer);

    // Hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 60);
    scores.insert(String::from("Red"), 70);
    scores.insert(String::from("Red"), 30); // This ovewrites the value above

    // Checking key before adding it
    scores.entry(String::from("Blue")).or_insert(20); // Won't ovewrite the value
    scores.entry(String::from("Magenta")).or_insert(96);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Prelude: Team {} score is {}", team_name, score);

    for (key, value) in &scores {
        println!("Team {} score is {}", key, value);
    }

    // Updating value based on old one
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Words count: {:?}", map);

    // Find Median
    println!("### Find Median & Mode ###");
    let mut values = vec![3, 4, 2, 0, 9, 5, 3, 4];
    let median = find_median(&mut values);
    println!("Median: {}", median);
    println!("Mode: {}", find_mode(&mut values));

    convert_pigin_latin(&String::from("first"));
    convert_pigin_latin(&String::from("apple"));
}


fn find_median(items: &mut Vec<i32>) -> i32 {
    items.sort();
    let half = items.len() / 2;
    if half % 2 == 0 {
        (items[half - 1] + items[half]) / 2
    } else {
        items[half - 1]
    }
}

fn find_mode(items: &mut Vec<i32>) -> i32 {
    let char = 'd';
    items.sort();
    let mut my_map = HashMap::new();
    for item in items {
        let count = my_map.entry(item).or_insert(0);
        *count += 1;
    }
    let ans = my_map
        .into_iter()
        .max_by_key(|x| x.1)
        .map(|x|x.0)
        .expect("Can't com");
    *ans
}

fn convert_pigin_latin(input: &String) {
    let mut chars = input.chars().peekable();
let mut new_s = String::new();
while let Some(c) = chars.next() {
    let suffix = match c {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            new_s.push(c);
            String::from("-hay")
        }
        'a'..='z' | 'A'..='Z' => {
            format!("-{}ay", c)
        }
        _ => {
            new_s.push(c);
            continue;
        }
    };

    while let Some(&c) = chars.peek() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                chars.next();
                new_s.push(c);
            }
            _ => break,
        }
    }

    new_s += &suffix;
}
println!("{}", new_s);
}
