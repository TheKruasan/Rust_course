use std::slice;

// unsafef function
unsafe fn dangerous() {}
// global variable
static HELLO_WORLD: &str = "Hello, world!";
// nutable global var
static mut COUNTER: u32 = 0;

// example of unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
fn main() {
    // raw pointers
    let mut num = 5;
    // immutable raw pointer
    let r1 = &num as *const i32;
    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    // creating a raw pointer to a random memory address
    let address = 0x012345usize;
    let r = address as *const i32;

    // dereferencing raw pointers with a unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // calling an unsafe function
    unsafe {
        dangerous();
    }

    // creating safe abstraction above unsafe code
    let mut vector = vec![1, 2, 3, 4, 5, 6];

    let (left, right) = split_at_mut(&mut vector, 3);

    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);

    // calling an extern function is unsafe
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // use of global var
    println!("name is: {}", HELLO_WORLD);

    // changing static var
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// takes one slice and makes it two by splitting the slice at the index given as an argument
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // unsafe block wrapped by safe function
    unsafe {
        (
            // takes a raw pointer and a length and it creates a slice
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
// extern function from C
extern "C" {
    // finds absolute value
    fn abs(input: i32) -> i32;
}

// changing static var is unsafe
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}