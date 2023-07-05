fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // example of using slice types
    let s = String::from("hello world");
    // first word
    let hello = &s[0..5]; // or &s[..5]
    // second word
    let world = &s[6..11]; // or &s[6..]

    //use slices to get the whole string
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
    let mut s = String::from("hello world");

    let word = first_word_sec(&s);
    // cant use mutable here
    // s.clear(); // error!

    println!("the first word is: {}", word);

    // string literal is string slice
    let s = "Hello, world!";

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_sec(&my_string[0..6]);
    let word = first_word_sec(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_sec(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_sec(&my_string_literal[0..6]);
    let word = first_word_sec(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_sec(my_string_literal);

    // array slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
// this function return index of first space in the string
fn first_word(s: &String) -> usize {
    // make a bytes array to iterate 
    let bytes = s.as_bytes();

    // iteration using the bytes array
    for (i, &item) in bytes.iter().enumerate() {
        // check to space
        if item == b' ' {
            // returns when space is found
            return i;
        }
    }

    // returns length of the string if we dont found the space
    s.len()
}


//this function returns the first word in the string
fn first_word_sec(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return slice from begin to first space
            return &s[0..i];
        }
    }

    // return the whole word if it's no spaces in string
    &s[..]
}



