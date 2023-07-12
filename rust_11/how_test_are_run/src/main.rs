// this function prints and returns 10 :)
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
// jus add two to number in params
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    // checks if prints_and_returns_10() function work correctly
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    // checks that add_two() function works correctly
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    // checks that add_two() function works correctly
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    // checks that add_two() function works correctly
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore] // that help us to ignore test
    fn expensive_test() {
        // code that takes an hour to run
    }
}