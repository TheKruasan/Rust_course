// defining a struct names rectangle
// this outer attribute implemets Debug method
#[derive(Debug)]//cant understand that fully but it helps...  
                //...us to print this struct to console 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // create variables that will be used to describes a rectangle
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // init rectangle with one value (tuple)
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );

    // init rectangle as struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    // print struct with debug format `{:?}`
    println!("rect1 is {:?}", rect1);

    // print struct with debug format `{:#?}` easier tor read
    println!("rect1 is {:#?}", rect1);

    // create a variable that will scale our rectangle
    let scale = 2;
    let rect1 = Rectangle {
        // prints debug info of this expression
        width: dbg!(30 * scale), // and give value and ownership back
        height: 50,
    };
    // prints debug info of struct
    dbg!(&rect1);
}

// This function calculate area of rectangle
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// This function calculate area of rectangle use the tuple that given as params
fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// This function calculate area of rectangle use the struct that given as parameter
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}