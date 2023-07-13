use std::thread;
use std::time::Duration;

// simulate workout according to user specified value and random number
fn generate_workout(intensity: u32, random_number: u32) {
    // expensive clousure with long time
    let expensive_closure = |num: u32| -> u32 {
        // annotated
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // control flow according due to params
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // init params for workout
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // test functuion
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // clousure that capture a reference
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    // list is still valid because we can have some immutable refs
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // cant print list here because clousure borrowed list
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // printing vector in new thread
    // using `move` to make list borrowed by clousure
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // list to sort
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut num_sort_operations = 0;
    // sorting by with param and counting operations
    list.sort_by_key(|r| { // this clousure is called not only one time
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}