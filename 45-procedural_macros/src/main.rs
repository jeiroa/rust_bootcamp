#![allow(dead_code, unused_variables)]

use procedural_macros::*; // import all items from the inner library

fn macro_function() {
    // macros can now be used appending a ! at the end of the function name
    log_info!(println!("Hello");); // replaced by "println!("Hello");" at compile time

    log_info!([TIME] starting program...);
}

trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Log)] // provide a default implementation of Log trait via our custom macro derive
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: String) -> Self {
        Self { url, connections: 0 }
    }

    fn connect(&mut self) {
        self.info(format!("new connection to {}", self.url).as_str());

        self.connections += 1;

        if self.connections >= 100 {
            self.warn(format!("100 or more connections open!").as_str());
        }
    }
}

fn macro_custom_derive() {
    let mut db = Database::new("localhost:5422".to_owned());

    for _ in 0..100 {
        db.connect();
    }
}

fn main() {
    macro_function();
    println!("-----------------------");
    macro_custom_derive();
}