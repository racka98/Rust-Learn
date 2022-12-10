use std::{fs::{File, self}, io::{ErrorKind, Read}};

#[allow(unused)]
pub fn error_handling() {
    // let v = vec![1, 2, 3];
    // v[99];

    // Opening a file
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Result::Ok(file) => file,
        Result::Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Result::Ok(file_created) => file_created,
                Result::Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let mut greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in the project");

    let mut file_text = String::new();
    greeting_file.read_to_string(&mut file_text);
    println!("File contents: {}", file_text);
    // Using fs
    let res = fs::read_to_string("hello.txt");
    match res {
        Result::Ok(s) => println!("Contents: {}", s),
        Result::Err(e) => println!("An error occurred: {:?}", e),
    }
}
