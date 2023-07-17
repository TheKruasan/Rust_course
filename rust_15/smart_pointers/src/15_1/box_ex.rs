pub fn storing_box() {
    let b = Box::new(5);//Storing an i32 value on the heap using a box
    println!("b = {}", b);
}