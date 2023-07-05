fn main() {

    let s1 = String::from("hello");

    // &s1 is reference to the value of s1 but does not own it
    let len = calculate_length(&s1);
    
    // thats why we have owned the s1 value 
    println!("The length of '{}' is {}.", s1, len);

    // Immutable reference
    let s = String::from("hello");

    //Creating a mutable string
    let mut s = String::from("hello");

    // Give mutable reference to some function in parameters
    change(&mut s);

    // we cant have more than one mutable reference to one value

    //example of using two mutable references
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // we can use more the one mutable refs if they in the different scopes
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    // We can create mutable ref in this exampe
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// This function calculate String length given as reference in params
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// This function change string given as mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// we need to return just value but not reference
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}