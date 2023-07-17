mod box_ex;
mod cons_list_ex;
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    box_ex::storing_box();
    cons_list_ex::cons_ex();
}