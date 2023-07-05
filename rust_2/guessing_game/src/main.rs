//import some libraries and Object oe functions in them
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    //create a random value using rand library that we will guess
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        
        //create a mutable string varible
        let mut guess = String::new();
        //read string guess by lines using io library
        io::stdin()
            //read line in string guess    
            .read_line(&mut guess)
            //if we cauch an exception print error message
            .expect("Failed to read line");
        //delete spaces and enters at the end and start of the string ...
        //... and parse the string value to u32 value ...
        //... if we have successfully do it we change value of string guess to it...
        //... else we continue programm 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        //compare our number with secret number
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