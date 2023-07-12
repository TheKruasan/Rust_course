use std::fs::File;
use std::io::ErrorKind;

fn main() {
    #![allow(unused)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    //open file
    let greeting_file_result = File::open("hello.txt");


    let greeting_file = match greeting_file_result {//if we succesfully read the file we return...
                                                          // the dictory of this file else we throw the error
        Ok(file) => file,
        Err(error) => match error.kind() {
            // if file not found we crate new file
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            //just throw the error
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };


}


#[allow(unused)]
//example of using expect
fn expectation() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}




use std::io::{self, Read};
//read string from file
fn read_username_from_file() -> Result<String, io::Error> {
    //read file
    let username_file_result = File::open("hello.txt");
    //if we can read file we read it, else return error
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    //declare new String
    let mut username = String::new();
    //read string from file and check it dor errors
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


//compact code entry above
fn read_username_from_file_sec() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
//compact code entry above
fn read_username_from_file_fird() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}


//compact code entry above
use std::fs;
#[allow(unused)]
fn read_username_from_file_fifth() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}