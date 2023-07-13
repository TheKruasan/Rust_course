fn main() {
    // separating the creation of the iterator from the use in the `for` loop
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    
}