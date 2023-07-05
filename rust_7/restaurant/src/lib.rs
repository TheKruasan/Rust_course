pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


//A front_of_house module containing other modules that then contain functions
mod front_of_house {//module
    mod hosting {//submodule
        fn add_to_waitlist() {}//function in module

        fn seat_at_table() {}//function in module
    }

    mod serving {//submodule
        fn take_order() {}//function in module

        fn serve_order() {}//function in module

        fn take_payment() {}//function in module
    }
}