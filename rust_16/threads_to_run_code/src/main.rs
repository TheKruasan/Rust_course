use std::thread;
use std::time::Duration;

fn main() {
    //createa thread using function spawn
    let handle = thread::spawn(|| {
        for i in 1..10 {//in thread we print numbers from 1 to 10 with latensy 1 millisecond
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // print numbers from 1 to 5 with latensy 1 millisecond
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();//wait when the spawned thread is be done

    //do it when spawned thread done his work
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }



    //Using move Closures with Threads

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {//in thread we cant use the variables from another thread
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // oh no!
    handle.join().unwrap();
}