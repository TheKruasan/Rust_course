// define a rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This function checks if rectangle given params can be placed in self
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// return number in params + 2
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// greets
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    // creates new Guess instance
    // panic if guess is not in [1, 100]
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// module with all tests
#[cfg(test)]
mod tests {
    // get all functions, structs to scope
    use super::*;

    // checks if larger rectangle can hold smaller
    #[test]
    fn larger_can_hold_smaller() {
        //creating rectangles
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    // checks that smaller rectangle can hold larger
    #[test]
    fn smaller_cannot_hold_larger() {
        //creating rectangles
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // checks that `add_two()` adds two to number
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // checks if greeting contains name given in params
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert macro with message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // checks if new instance of Guess in bounds [1, 100]
    #[test]
    // code can panic and we expect that panic message will contain defined message
    #[should_panic(expected = "less than or equal to 100")] // else test will fail
    fn greater_than_100() {
        Guess::new(200);
    }

    // we also can use Result<T, E> as result of test 
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}