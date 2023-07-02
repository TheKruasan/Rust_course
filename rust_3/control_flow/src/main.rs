
fn main(){
    //function with few if controls
    multiple_if();
    //iteration on array with while and index
    array_iteration();
    //iteration in array with for
    another_array_iteration();
    //example of using loop for
    loop_for();
    //example of using while
    endless_loop();
}

fn multiple_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


fn array_iteration() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn another_array_iteration() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn endless_loop() {
    loop {
        println!("again!");
    }
}