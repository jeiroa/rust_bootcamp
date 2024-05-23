#![allow(dead_code, unused_variables)]

// main function to test external modules
use modules::Credentials; // import both and Credentials from lib.rs

fn main() {
    let creds = Credentials::new("user".to_owned(), "pass".to_owned());

    modules::authenticate(creds);
}