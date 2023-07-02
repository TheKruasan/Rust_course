fn main() {
    println!("Hello, world!");

    another_function();
    
    // give params to function
    another_function_with_params(5);

    print_labeled_measurement(5, 'h');

    // using a statement for binding cause error
    // let x = (let y = 6);

    // using an expression doesnt cause error
    // because it has a returnable value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");


    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_params(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// return value is the last expression in brackets
fn five() -> i32 {
    5
}

// dont use semicolon in last expression if it is return value
fn plus_one(x: i32) -> i32 {
    x + 1
}
