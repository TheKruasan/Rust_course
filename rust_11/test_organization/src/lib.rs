// add two calling private function
// wrapper, that helps to test private function
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// private function that adds two to the number
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // brings private function to scope
    use super::*;

    // tests private function
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}