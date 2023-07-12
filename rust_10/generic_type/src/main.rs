fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}



// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main_2() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

//--In Struct Definitions--//
struct Point<T> {
    x: T,
    y: T,
}

fn main_sec() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // let wont_work = Point { x: 5, y: 4.0 };//there tre error because type of x - T...
                                                       //... and we say that x - integer thats why...
                                                       //... the T - integer ,and there we try to give y a float value
}
//correct code above
struct PointF<T, U> {
    x: T,
    y: U,
}

fn main_third() {
    let both_integer = PointF { x: 5, y: 10 };
    let both_float = PointF { x: 1.0, y: 4.0 };
    let integer_and_float = PointF { x: 5, y: 4.0 };
}

//--In Enum Definitions--//
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}


//--In Method Definitions--//
struct PointS<T> {
    x: T,
    y: T,
}
impl<T> PointS<T> {
    fn x(&self) -> &T {
        &self.x
    }

}
impl PointS<f32> {//method that we can use if T - f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main_fifth() {
    let p = PointS { x: 5, y: 10 };
    //method call
    println!("p.x = {}", p.x());
}




// A method that uses generic types different from its struct’s definition
struct Point_S<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point_S<X1, Y1> {
    //use 1 value of self struct and take 2 value of second struct
    fn mixup<X2, Y2>(self, other: Point_S<X2, Y2>) -> Point_S<X1, Y2> {
        Point_S {
            x: self.x,
            y: other.y,
        }
    }
}

fn main_r() {
    //create instances of structs
    let p1 = Point_S { x: 5, y: 10.4 };
    let p2 = Point_S { x: "Hello", y: 'c' };
    //method call
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

//--Performance of Code Using Generics--//

fn last(){
    let integer = Some(5);
    let float = Some(5.0);


    
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn last_s() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}