#![allow(dead_code, unused_variables)]
use std::{fs, io};

fn read_first_line(filename: &str) -> Result<String, io::Error> {
    // read whole file
    fs::read_to_string(filename)
        .and_then(|s| {
            s.lines()
                .next()
                .map(|s| s.to_owned()) // it fails because an Option<String> is returned instead of a Result
        })
}

// first solution is to wrap the value to return into an Option to be returned in the Results
fn read_first_line2(filename: &str) -> Result<Option<String>, io::Error> {
    // read whole file
    fs::read_to_string(filename)
        .map(|s| { // so map method is a combinator needs to be used instead of and_then
            s.lines()
                .next()
                .map(|s| s.to_owned())
        })
}

// second solution is to extract the correct value
fn read_first_line3(filename: &str) -> Result<String, io::Error> {
    // read whole file
    fs::read_to_string(filename)
        .map(|s| {
            s.lines()
                .next()
                .map(|s| s.to_owned())
                .unwrap_or_default() // so not return Option but the actual value
        })
}

// third solution is to convert between Option and Result types and change method signature
fn read_first_line4(filename: &str) -> Option<String> {
    // read whole file
    fs::read_to_string(filename)
        .ok() // combinator to convert a Result into an Option
        .and_then(|s| {
            s.lines()
                .next()
                .map(|s| s.to_owned())
        })
}

fn main() {
    let first_line = read_first_line("example.txt");
}
