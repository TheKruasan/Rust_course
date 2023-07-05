fn main() {
    //define a enum named coin with some types of data
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    //function that get a instance of Coin in params and return it's value
    fn value_in_cents(coin: Coin) -> u8 {
        //example of using match control flow
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    //second way of function value_in_cents but match can return a function
    fn sec_value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }


    #[derive(Debug)] // so we can inspect the state in a minute
    //defining the enum named UsState
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),//changing the Quarter variant to include a UsState value stored inside it
    }

    //third way of function value_in_cents but match can handle new enum 
    fn third_value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {//handle a new type of data
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    //function that takes an Option<i32> and, if there’s a value inside, ...
    //... adds 1 to that value. If there isn’t a value inside, the function ... 
    //... should return the None value and not attempt to perform any operations.
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,//return None
            Some(i) => Some(i + 1),//return a value and add 1 to it
        }
    }
    //create a instace of Option enum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }


    //we can't use this code because match have to describe all variants of variable
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // end of error code
    placehold()
    
}

def placehold(){

    let dice_roll = 9;

    // if we have not value 3 or 7 we use move_player() function
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }


    //if we need'nt use a value of math we can use pattern '_'
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }




    //we aren’t going to use any other value that doesn’t match ...
    //... a pattern in an earlier arm, and we don’t want to run any code in this case
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }




    //three functions for test code above
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}