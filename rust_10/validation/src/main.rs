use std::fmt::Display;

// a struct with a reference field
struct ImportantExcerpt<'a> {
    // in that case we should define a lifetime
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    // dont need to define a lifetime according to first rule
    fn level(&self) -> i32 {
        3
    }
    // dont need to define a lifetime according to third rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
                          // ----------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // &i32 - a reference
    // &'a i32  - a reference with an explicit lifetime
    // &'a mut i32 - a mutable reference with an explicit lifetime

    // in this example string2 has less lifitime than string1
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        // result lifitime is important too
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // example of using struct with lifetime 
    let novel = String::from("Call me Ishmael. Some years ago...");
    // get first sentence as str slice
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // creating instance with defining a lifetime
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static reference that live all time while program is working
    let s: &'static str = "I have a static lifetime.";

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

// This function finds the largest string given in params
// 'a means that both params` lifetimes are not less than `a lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// this function returns a longest word given in params and print announcement
// example of using generics, trait bound and lifetime

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T, // this param should be displayable
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}