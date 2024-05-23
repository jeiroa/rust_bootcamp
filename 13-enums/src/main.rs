#![allow(dead_code, unused_variables)]

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct Product_v1 {
    name: String,
    category: String,
    price: f32,
    in_stock: bool,
}

// simple enum, just a list of values
#[derive(Debug)]
enum Category {
    Books,
    Clothing,
    Electrics,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct Product_v2 {
    name: String,
    category: Category, // Category is now a list of predefined values
    price: f32,
    in_stock: bool,
}

// complex enum where some values have data asocciated
#[derive(Debug)]
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    },
}

// it is also possible to add implementation functions to enums
impl Command {
    // convert the command data into a String
    fn serialize(&self) -> String {
        // match result can be stored in a variable
        let json_string = match &self {
            Command::Undo => String::from(
                "{ \"cmd\": \"undo\" }"
            ),
            Command::Redo => String::from(
                "{ \"cmd\": \"redo\" }"
            ),
            Command::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            },
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            },
            Command::Replace { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
        };
        
        json_string
    }
}

fn match_expressions() {
    let age = 35;

    // match could return a value to be stored in a variable
    match age {
        1 => println!("Happy 1st birthday!"),
        13..=19 => println!("You are a teenager!"),
        x => println!("You are {x} years old"), // else
        //_ => println!(""), // or do nothing for everything else
    }
}

fn main() {
    let product1 = Product_v1 {
        name: String::from("Radio"),
        category: String::from("Something"),
        price: 20.35,
        in_stock: true
    };

    println!("Product v1: {:?}", product1);

    let category = Category::Books; // access to enum value like statuc
    let product2 = Product_v2 {
        name: String::from("TV"),
        category: category,
        price: 200.98,
        in_stock: true
    };

    println!("Product v2: {:?}", product2);

    // use complex enum values
    let command1 = Command::Undo;
    let command2 = Command::AddText(String::from("test"));
    let command3 = Command::MoveCursor(22, 0);
    let command4 = Command::Replace {
        from: String::from("a"),
        to: String::from("b")
    };

    println!("Command: {:?}", command1);
    println!("Command: {:?}", command2);
    println!("Command: {:?}", command3);
    println!("Command: {:?}", command4);

    println!("Serialized command: {}", command1.serialize());
    println!("Serialized command: {}", command2.serialize());
    println!("Serialized command: {}", command3.serialize());
    println!("Serialized command: {}", command4.serialize());

    match_expressions();
}
