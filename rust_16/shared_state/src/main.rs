use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

fn main() {
    let m = Mutex::new(5);//create mutex   

    {
        let mut num = m.lock().unwrap();//pick the value from mutex
        *num = 6;//change value in mutex
    }

    println!("m = {:?}", m);//print value in mutex

}

fn counter_ex(){
    let counter = Arc::new(Mutex::new(0));//create mutex
    let mut handles = vec![];//create vector of threads

    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {//we cant use the data from mutex in few threads
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }


    for _ in 0..10 {
        let counter = Arc::clone(&counter);//clone our counter
        let handle = thread::spawn(move || {//in every iteration we spawn new thread
            let mut num = counter.lock().unwrap();//pick the calue from mutex

            *num += 1;//change it and then lock it
        });
        handles.push(handle);//push the thread in vector
    }




    for handle in handles {
        handle.join().unwrap();//wait when all threads do their work
    }

    println!("Result: {}", *counter.lock().unwrap());
}