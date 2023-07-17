use std::ops::Deref;


fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);//Using the dereference operator to follow a reference to an i32 value

    let x = 5;
    let y = Box::new(x);//Using the dereference operator on a Box<i32>

    assert_eq!(5, x);
    assert_eq!(5, *y);

    struct MyBox<T>(T);//Defining a MyBox<T> type

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }


    let x = 5;//Attempting to use MyBox<T> in the same way we used references and Box<T>
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);


    impl<T> Deref for MyBox<T> {//Implementing Deref on MyBox<T>
        type Target = T;
    
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }


    fn hello(name: &str) {//A hello function that has the parameter name of type &str
        println!("Hello, {name}!");
    }
    let m = MyBox::new(String::from("Rust"));//The code we would have to write if Rust didn’t have deref coercion
    hello(&(*m)[..]);
}