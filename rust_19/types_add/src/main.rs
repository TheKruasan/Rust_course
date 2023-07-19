use std::cmp::Ordering;
use std::io;
fn main() {
    // creating synonymus type
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    // they are the same type
    println!("x + y = {}", x + y);

    // use synonymus to make type names shorter and more convinient
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));
    // exmple of using new typename
    fn takes_long_type(f: Thunk) {}

    print!("forever ");
    // loop has a never return type
    loop {
        print!("and ever ");
        break;
    }

    // give function in params
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // can use closures and functions in map method
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }
    // can use these initializer functions as function pointers that implement the closure traits
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// taken from chapter two
// pay attention to continue return type
fn guess_game() {
    println!("Guess the number!");

    let secret_number = 5;

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // continue here returns a never type
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// if we dont know T size at the moment of compilation
// we use ?Sized, and then use &T
fn generic<T: ?Sized>(t: &T) {}

fn add_one(x: i32) -> i32 {
    x + 1
}
// takes function in params and do it twice
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// returns closures using pointers
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}