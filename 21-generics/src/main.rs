#![allow(dead_code, unused_variables)]

struct BrowserCommand<T> {
    name: String,
    payload: T,
}

// generics can also be used with enumerations
// see Option<T> and Result<T, E> enums

// generics has to be added after impl for no concrete type implementations
impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand{name, payload}
    }

    // return a generic
    fn get_payload(&self) -> &T {
        &self.payload
    }
}

// concrete type (String) implementation
// this implementation is only valid for String payloads
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

// list of generic types to be used in a function is specified right after the name between <>
// in this case, the compiler will create two functions based on its usage (monomorphization)
// fn serialize_payload_string(payload: String) -> String
// fn serialize_payload_i32(payload: i32) -> String
// so there is no performance loss
fn serialize_payload<T>(payload: T) -> String {
    String::from("Test placeholder")
}

fn main() {
    let cmd1 = BrowserCommand {
        name: String::from("navigate"),
        payload: String::from("https://www.letsgetrusty.com")
    };

    let cmd2 = BrowserCommand::new(
        String::from("zoom"),
        200,
    );

    cmd1.print_payload();
    // cmd2.print_payload(); //this won't compile

    // printing the payload from generic return type
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();
    println!("{}", p1);
    println!("{}", p2);

    // generic function is called with different types
    println!("{}", serialize_payload(p1));
    println!("{}", serialize_payload(p2));
}
