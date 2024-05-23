#![allow(dead_code, unused_variables)]

fn main() {
    // macro to thrown a panic error which abruptly ends the program
    //panic!("Unrecoverable fatal error!!!");

    // runtime panic
    let v = vec!["one", "two", "three"];
    print!("{}", v[3]); // index out of bounds panic
}
