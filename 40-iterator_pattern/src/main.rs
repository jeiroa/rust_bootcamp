#![allow(dead_code, unused_variables)]
use std::collections::HashMap;
// Iterator trait has an associated type meaning that only one type of element can be returned by implementation
// If a generic were used the several implementations would be allowed
// associated types in traits that restrict to just one implementation per component
struct MyStruct {}

trait IteratorReal {
    type Item; // associated type

    fn next(&mut self) -> Option<Self::Item>;
}

trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

//impl IteratorReal for MyStruct {
//    type Item = String;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        None
//    }
//}
// Generics allow several implementations for a component (we sometimes do not want it)
impl IteratorGeneric<String> for MyStruct {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl IteratorGeneric<i32> for MyStruct {
    fn next(&mut self) -> Option<i32> {
        None
    }
}

struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>,
}

// implementation that returns the next element in the iterator
impl Iterator for PersonIterator {
    type Item = String; // element type

    // what element is nex?
    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None
        }
        Some(self.values.remove(0))
    }
}

// implementation that creates and returns the iterator container
impl IntoIterator for Person {
    type Item = String; // element type
    type IntoIter = PersonIterator; // iterator type

    fn into_iter(self) -> Self::IntoIter {
        // create a new iterator container with an array with the content of the iterator state
        PersonIterator {
            values: vec![
                self.first_name,
                self.last_name,
                self.occupation
            ]
        }
    }
}

fn main() {
    let mut m = MyStruct {};
    // it would be necessary to explicity set the return type if generics are used
    let item1: Option<String> = m.next();
    let item2: Option<i32> = m.next();
    // for the real Iterator trait is it not necessary because it does not allow more than one implementation
    //let item1 = m.next();

    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Software Engineer".to_owned()
    };

    let mut iter = p.into_iter();
    println!("{:?}", iter.next()); // Some("John")
    println!("{:?}", iter.next()); // Some("Doe")
    println!("{:?}", iter.next()); // Some("Software Engineer")
    println!("{:?}", iter.next()); // None

    let p2 = Person {
        first_name: "Jane".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Project Manager".to_owned()
    };

    // p2 is of type Person which implements Iterator trait so the actual String value is printed (not Optional)
    for item in p2 {
        println!("{item}"); // Jane, Doe, & Project Manager
    }

    // Iterators are used to perform operations in Collections
    let mut scores = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 8);
    scores.insert("green team".to_owned(), 6);

    // this one uses iter() method on scores which returns a Iter with immutable references
    for (team, score) in &scores { // &scores --> get immutable references
        println!("Team: {}; Score: {}", team, score);
    }

    // this one uses iter_mut() method on scores which returns an IterMut with mutable references (only available on values)
    for (team, score) in &mut scores { // &mut scores --> get mutable references
        println!("Team: {}; Score: {}", team, score);
    }

    // this one uses into_iter() method which returns owned values
    for (team, score) in scores { // scores --> get immutable owned values
        println!("Team: {}; Score: {}", team, score);
    }
}
