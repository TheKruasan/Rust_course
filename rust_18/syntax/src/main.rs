fn main() {
    let x = 1;
    //group of patterns
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    //Ranges of Values 
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    //ranges wuth char type
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // --Destructuring Structs-- //
    

    //create Point struct with two atributtes
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };// create entity of the struct

    let Point { x: a, y: b } = p;// Destructuring a struct’s fields into separate variables
    assert_eq!(0, a);
    assert_eq!(7, b);


    
    //create Point struct with two atributtes
    struct Point {
        x: i32,
        y: i32,
    }
    
    fn main() {
        let p = Point { x: 0, y: 7 };// create entity of the struct
    
        let Point { x, y } = p;//Destructuring struct fields using struct field shorthand
        assert_eq!(0, x);
        assert_eq!(7, y);
    }



}

// Destructuring and matching literal values in one pattern
fn destruct_struct() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destruct_enum() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));//create variable wuth emun type
    //destruct dif types with match
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        _ => (),
    }

}


//dectruct structs and tuples
fn dectruct_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

//Ignoring Values in a Pattern
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn foo_main() {
    foo(3, 4);
}


//Ignoring Parts of a Value with a Nested _
fn irgore_parts() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {//Using an underscore within patterns that match Some ...
                               //variants when we don’t need to use the value inside the Some
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    //also we can ignore not everything
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

//Ignoring an Unused Variable by Starting Its Name with _
fn ignore_x_not_y() {
    let _x = 5;
    let y = 10;
}

//Ignoring Remaining Parts of a Value with ..

fn ignore_with_two_dots() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),//ignore value after x
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {//ignore value between first and last 
            println!("Some numbers: {first}, {last}");
        }
    }
}


//Match guard
pub fn match_guards(){
    let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
}

//@ Bindings
fn bind() {
    enum Message {//create enum
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };//create value with enum type

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,//if id at 3 to 7 
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}