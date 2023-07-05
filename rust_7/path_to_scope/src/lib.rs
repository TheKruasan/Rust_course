//module tree
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
//use some libs
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
//use some libs
use std::fmt;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}