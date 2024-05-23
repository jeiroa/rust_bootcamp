#![allow(dead_code, unused_variables)]

use std::{error, fs, num};

fn main() {
    match parse_file1("example.txt") {
        Ok(i) => println!("Number {i} was correctly read"),
        Err(err) => println!("Error processing the file: {err}"),
    }

    match parse_file2("example.txt") {
        Ok(i) => println!("Number {i} was correctly read"),
        Err(err) => match err {
            ParseFileError::File => println!("Error reading the file"),
            ParseFileError::Parse(int_error) => println!("Error parsing the content: {int_error}"),
        }
    }
}

// so we cannot return io::Error but a common error.
// both errors implement Error trait so we switch from io::Error to error::Error
// a Box pointer to any implementation of that type is necessary (remember trait bounds)
// this solution is easy to implements but clients will not be able to handle concrete errors
fn parse_file1(filename: &str) -> Result<i32, Box<dyn error::Error>> {
    let s = fs::read_to_string(filename)?; // this returns io:Error
    let i = s.parse()?; // while this one returns std::num::ParseIntError
    Ok(i)
}

// custom error to be returned
#[derive(Debug)]
enum ParseFileError {
    File, // in case on error reading the file
    Parse(num::ParseIntError) // in case of error parsing the content
}

// a second solution is to return an error as a custom enumeration
fn parse_file2(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename)
        .map_err(|e| ParseFileError::File)?;
    let i = s.parse()
        .map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

// custom Errors should implement error:Error trait so they should be enums or structs
// example of an error struct, used when more context is necessary but the source error can be generic
struct ServerError {
    status_code: u8, // the HTTP status code
    body: String, // error message string
    source: Box<dyn error::Error>, // underlying error
}

// enum is preferred when it si necessary to provide details about the underlying error
// this is the most used aproach
enum APIError {
    UserInputError(String),
    InternalError(Box<dyn error::Error>),
}

// mixed approach
enum APIErrorType {
    UserInputError,
    InternalError,
}

struct APIMixError {
    msg: String,
    source: Option<Box<dyn error::Error>>,
    err_type: APIErrorType,
}