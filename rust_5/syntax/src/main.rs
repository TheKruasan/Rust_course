#[derive(Debug)]
// define a struct named rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation of rectangle struct
impl Rectangle {

    //This method calculate area of rectangle
    fn area(&self) -> u32 { 
        self.width * self.height 
    }
    // returns true if width is positive
    fn width(&self) -> bool { // able to use the same name method as struct field
        self.width > 0
    }
    // returns true if rectanale can hold the rectangle given in params
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // associated function that creates a new instance of struct
    fn square(size: u32) -> Self {
        Self {
            // square has equal width and height
            width: size,
            height: size,
        }
    }
}

fn main() {
    // creating a rectangles
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // call method with dot notation
        rect1.area()
    );

    // example of using method named like struct field
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // example of parameterized method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // call associated function
    let sq = Rectangle::square(3);
}