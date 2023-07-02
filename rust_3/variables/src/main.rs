fn main() {
    // create mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    // changing the value of mutable variable
    x = 6;
    println!("The value of x is: {x}");

    // create const variable 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // example of shadowing
    let x = 5;
    let x = x + 1;
    // shadowing a variable in rackets
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // changing value of variable in brackets didnt affect on value outside of brackets
    println!("The value of x is: {x}");


    let spaces = "Hello world!!!";
    println!("The value of spaces is: {spaces}");
    // change a type of variable
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}