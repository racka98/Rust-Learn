use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn concurrency() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Using Move to capture values from main thread
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    // Sharing between threads
    share_between_threads();

    // Mutexes for accessing same data between threads
    using_mutex();
}

fn share_between_threads() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for value in vals {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for value in vals {
            tx1.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn using_mutex() {
    println!(" ***** Mutex Example ******");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}