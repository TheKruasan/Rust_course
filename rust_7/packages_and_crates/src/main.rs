use crate::garden::vegetables::Asparagus;

pub mod garden; //The pub mod garden; line tells the compiler ...
                //... to include the code it finds in src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}