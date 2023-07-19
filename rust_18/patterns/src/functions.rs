pub fn while_let() {
    //create vector
    let mut stack = Vec::new();
    //push some value in vector
    stack.push(1);
    stack.push(2);
    stack.push(3);
    //while we have top value we print it ...
    //... when the value will be None the loop stops it work
    //while let pattern
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

#[allow(unused)]
pub fn use_dif_constructions_with_if_let_and_else() {
    // Mixing if let, else if, else if let, and else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    //if let pattern
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

//function pattern
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}