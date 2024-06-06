use std::collections::HashMap;

use declarative_macros::{ hello, map };

// function that creates a Map of teams and the score associated
fn create_map() {
    // create a HashMap directly
    let scores: HashMap<String, i32> = HashMap::new();

    println!("{:?}", scores);
    // use a macro to create it
    let scores = map!(String, i32);

    println!("{:?}", scores);
}

fn create_populate_map() {
    let mut scores = HashMap::new();
    scores.insert("Read team".to_owned(), 3);
    scores.insert("Blue team".to_owned(), 5);
    scores.insert("Green team".to_owned(), 2);

    println!("{:?}", scores);

    let scores = map!(
        "Read team".to_owned() => 3,
        "Blue team".to_owned() => 5,
        "Green team".to_owned() => 2
    );

    println!("{:?}", scores);
}

fn main() {
    hello!(); // both () and [] work because we only need a grouping token being both of them one
    hello![]; // [] makes more sense if a list of items is expected

    create_map();

    create_populate_map();
}
