use crate::List::{Cons, Nil};

pub fn cons_ex() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}",list);
}