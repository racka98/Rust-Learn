use crate::smart_pointers::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

pub fn smart_pointers() {
    // Using Box to store data
    let b = Box::new(5);
    println!("b: {}", b);

    // Using cons list
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref trait
    let m = MyBox::new(String::from("Rustlang"));
    hello(&m);

    // Drop trait should print the text inside drop fn
    let c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("Other Stuff"),
    };
    drop(d);
    println!("CustomSmartPointers created!!!");

    // Reference Counting
    let x = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    println!("count after creating x: {}", Rc::strong_count(&x));
    let y = ListRc::Cons(3, Rc::clone(&x));
    println!("count after creating y: {}", Rc::strong_count(&x));
    {
        let z = ListRc::Cons(4, Rc::clone(&x));
        println!("count after creating z: {}", Rc::strong_count(&x));
    }
    println!("count after z goes out of scope: {}", Rc::strong_count(&x));

    // Interior Mutability with RefCell
}

// cons list
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

// Our own Box impl
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Using Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// RefCell use case
trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: u32,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: u32) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::smart_pointers::MyBox;
    use super::*;

    #[test]
    fn test_box_references() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    
    // Using Mocks
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}