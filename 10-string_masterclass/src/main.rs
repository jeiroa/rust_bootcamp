#![allow(dead_code, unused_variables)]

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    create_strings();
    manipulate_strings();
    concatenate_strings();
    string_indexing();
    iterate_string();
    strings_in_functions();
}

fn create_strings() {
    let s1 = "Привіт Світ! 🦀"; // create a string slice in the binary memory
    let s2 = String::from("Привіт Світ!"); // create a String from a string slice
    let s3 = "Привіт Світ!".to_string(); // this is the same as above
    let s4 = "Привіт Світ!".to_owned(); // this is the same as String::from

    // UTF-8 chars are made up of 1 up to 4 bytes
    println!("{}", &s1[0..2]); // П char is stored in bytes 0 and 1 (2 bytes). Taking only 0 will panicked
    println!("{}", &s1[21..22]); // ! char is just one byte
    println!("{}", &s1[23..27]); // crab char is actually 4 bytes
}

fn manipulate_strings() {
    let mut s = String::from("foo");
    s.push_str("bar"); // String contatenation
    println!("{}", s);
    s = s.replace("bar", "fee"); // notice the original string is not modified
    println!("{}", s);
    s.replace_range(3.., "bar"); // replace a range
    println!("{}", s);
}

fn concatenate_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // + operator
    let s3 = s1 + &s2; // concatenate s1 with a reference of s2 and move to s3
    // println!("s1: {}", s1); <-- cannot use s1 anymore as it was moved
    println!("s2: {}", s2); // s2 was not moved (a reference was used)
    println!("s3: {}", s3);

    // format! macro
    // it is less efficient because strings are cloned
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
    println!("s1: {}", s1); // as they were cloned they are already accesible
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    // concat array of string slices into a String
    let s1 = ["first", "second"].concat();
    // use format! macro to also concat string slices into a String
    let s2 = format!("{}{}", "first", "second");
    // use concat! macro to concat string slices into another string slice
    let s3 = concat!("first", "second");
    // use + to concat a String to a string slice
    let s4 = String::from("test");
    let s5 = s4 + "okok" + &String::from("okok"); // String var must be the first one and the rest &str
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    //println!("s4: {}", s4); <-- s4 was moved when it was concatenated to create s5
    println!("s5: {}", s5);
}

fn string_indexing() {
    let s1 = "🦀🦀🦀🦀🦀";
    //let s2 = s1[0]; <-- it is not possible to do this because it is not known how many bytes has every char
    let s2 = &s1[0..4]; // however, we know that every crab char is 4 bytes so string slice can be used
    println!("s2: {}", s2);
}

fn iterate_string() {
    // iterate over the bytes of a string slice (18 in this case)
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // iterate over the unicode scalar values which could not  match the UTF-8 ones (in this case 6 chars are returned)
    for c in "नमस्ते".chars() {
        println!("{}", c); // स् and ते are made up of 2 unicode scalar values
    }

    // it is necessary to use the unicode-segmentation crate to correctly iterate over a UTF-8 string
    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }
}

fn strings_in_functions() {
    let s1 = "Hellow World! one";
    let s2 = String::from("Hellow World! two");

    my_function(s1); // pass a string slice
    my_function(&s2); // pass a reference to a String

    // both can be used again because they were not moved
    println!("{}", s1);
    println!("{}", s2);
}

// it is recommended to pass &str to functions because references to String can also be used thanks to deref coercion
fn my_function(a: &str) -> String {
    return format!("{}", a);
}