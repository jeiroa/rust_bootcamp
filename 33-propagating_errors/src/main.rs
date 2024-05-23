#![allow(dead_code, unused_variables)]
use std::{fs::File, io::{self, Read}};

struct User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: User) -> Option<String> {
    // ? also works with Option, returning the value of Some or None
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;

    Some(format!("{first_initial}.{last_initial}"))
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // ? unwraps the Result if Ok (String) or propagates the Err if not (io::Error)

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let contents = read_file("example.txt");

    let initial = get_initials(User { firstname: "Pepe".to_owned(), lastname: "Luís".to_owned() });
}
