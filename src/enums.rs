#![allow(unused)]
pub fn enums_actions() {
    let four = IpAddrKind::V4(String::from("1.1.1.1"));
    let six = IpAddrKind::V6(String::from("0.0.0.0"));

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };

    let write_msg = Message::Write(String::from("Hello Message"));
    write_msg.call();

    // The Option enum
    let some_number = Some(20);
    let some_chart = Some('b');
    let absent_number: Option<i32> = None;
    let new_num = some_number.unwrap_or(0) + 3;
    println!("New Number: {}", new_num);

    // if let syntax
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The absolute maximum is {}", max),
        _ => ()
    }

    // Above definition can also be written as
    let config_max = Some(5u8);
    if let Some(max) = config_max {
        println!("The absolute maximum is {}", max);
    }

    if let Message::Write(msg) = write_msg  {
        println!("My Message is: {}", msg);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/**
 * String here represents the address
 */
enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quiting"),
            Message::Move { x, y } => println!("Moving: x = {}, y = {}", x, y),
            Message::Write(message) => println!("Writing Msg: {}", message),
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to: R={}, G={}, B={}", r, g, b)
            },
            other => println!("Some other"),
        };
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
