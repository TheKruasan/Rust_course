use adder;
mod common;
// adds two
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}