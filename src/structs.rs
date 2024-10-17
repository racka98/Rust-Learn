struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_standing(&self) -> bool {
        self.height > self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Fn that doesn't use self
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Tupple struct
struct Point(i32, i32, i32);

// Unit-like sturcts
#[derive(Debug)]
struct AlwaysEqual;

pub fn stuct_test() {
    let mut user1 = User {
        email: String::from("rackapps@gmail.com"),
        username: String::from("racka"),
        active: true,
        sign_in_count: 20,
    };
    user1.email = String::from("another@example.com");
    display_userdata(&user1);
    let user2 = build_user(String::from("hello@gmail.com"), String::from("rackadev"));
    display_userdata(&user2);

    // Struct update syntax. We create user3 from user2
    // Note: This "moves" the data from user2
    // But it wouldn't be moved if we assigned both email and username.
    // The left active and sign_in_count implement the Copy trait so they will leave the user2 valid
    let user3 = User {
        email: String::from("user3@email.com"),
        ..user2 // This should come last
    };
    display_userdata(&user3);

    // Tupple struct
    let point1 = Point(0, 3, 1);
    println!(
        "Pont values: x = {}, y = {}, z = {}",
        point1.0, point1.1, point1.2
    );

    // Unit-like struct
    let subject = AlwaysEqual;
    println!("Unit like: {:?}", subject);

    println!("### Structs: Areas of a Triangle");
    rectangle_actions();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn display_userdata(user: &User) {
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Is Active: {}", user.active);
    println!("Sign In Count: {}", user.sign_in_count);
}

fn rectangle_actions() {
    let width = 30;
    let height = 70;
    let rect1 = Rectangle { width, height };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} Square Pixels",
        rect1.area()
    );
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
    println!("Is rectangle standing: {}", rect1.is_standing());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    let sqr1 = Rectangle::square(30);
    println!("Square 1 is {:?}", sqr1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 4,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 4,
            height: 5,
        };
        assert!(!smaller.can_hold(&larger));
    }
}
