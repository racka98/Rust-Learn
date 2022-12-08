pub fn variable_ownership() {
    println!("Hello ownership");
    let mut s = String::from("Hello String!");
    s.push_str(", This is Extra");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("Some string");
    let s2 = s1.clone(); // Clone to make a deepcopy instead of move
    let s3 = String::from("S3 ownership");
    take_ownership(s3);
    println!("S1: {}, S2: {}", s1, s2);

    println!("### References & Borrowing ###");
    references_borrowing();
    println!("### Slice Type ###");
    let sentence = String::from("you there");
    let word = first_word(&sentence);
    println!("First Word: {word}");
    let sn = String::from("Hello world");
    let hello = &sn[0..=4];
    let world = &sn[6..];
    println!("S1: {hello}. S2: {world}");
}

fn take_ownership(some_string: String) {
    println!("Taking owneship: {}", some_string);
}

fn references_borrowing() {
    let s1 = String::from("hello");
    let length = caluclate_length(&s1);
    println!("The length of {s1} is {length}")
}

fn caluclate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..] // returns the entire slice
}
