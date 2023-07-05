//use librarys
use std::collections::*;
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let mut map = HashMap::new();//create new HashMap
    map.insert(1, 2);//add to HashMap some data

    let secret_number = rand::thread_rng().gen_range(1..=100);//create random value
}