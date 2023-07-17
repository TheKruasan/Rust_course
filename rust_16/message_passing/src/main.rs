use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // create channel "many producers, single customer"
    let (tx, rx) = mpsc::channel();
    // transmitter for second producer
    let tx1 = tx.clone();
    // create first thread-producer
    thread::spawn(move || { // using move to make thread own transmitter
        // some messages
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        // sending messages
        for val in vals {
            // send
            tx1.send(val).unwrap();
            // wait
            thread::sleep(Duration::from_secs(1));
        }
    });
    // create second thread-producer
    thread::spawn(move || {
        // some messages
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
         // sending messages
        for val in vals {
            tx.send(val).unwrap();  // send
            thread::sleep(Duration::from_secs(1));  // wait
        }
    });
    // recieving messages
    for received in rx {
        println!("Got: {}", received);
    }

}