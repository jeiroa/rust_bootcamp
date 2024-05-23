#![allow(dead_code, unused_variables)]
use std::fs::File;

// Result enum is used to handle recoverable errors

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err(String::from("Username cannot be empty"))
    } else {
        Ok(1) // return 1 always
    }
}

fn main() {
    let file = File::open("example.txt");

    let file = match file {
        Ok(file) => file, // if Ok, the file variable contains the file and it is assigned to file var
        Err(error) => { // if Error, the error variable contains the error
            panic!("Failed to open file {:?}", error)
        }
    };

    // this is the same as above
    let file = File::open("example.txt").unwrap();
    // same as above but a custom error message is provided
    let file = File::open("example.txt").expect("Panic message in case of error");

    // there are many other helper methods in Result
}
