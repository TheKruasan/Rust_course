mod functions;

fn main() {
    //pattern of binding
    let x = 5;
    //match patern
    println!("Hello, world!");
    let x = Option::Some(1);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    functions::while_let();
}


