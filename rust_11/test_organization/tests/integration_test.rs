use test_organization;
mod common;
// adds two
#[test]
fn it_adds_two() {
    assert_eq!(4, test_organization::add_two(2));
}