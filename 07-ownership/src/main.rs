fn main() {
    normal();
    move_ownership();
    clone_ownership();
    primitives_ownership();

    function_ownership();
}

fn normal() {
    let s1 = String::from("Rust"); // string allocated on the heap but s1 pointer is on the stack

    println!("s1 is: {s1}");
}

fn move_ownership() {
    let s1 = String::from("Rust");
    let s2 = s1; // now s2 points to the value s1 was pointing so s1 is no longer valid

    // println!("s1 is {s1}"); <-- error borrow of move value: s1 (s2 borrowed s1 so s1 was removed)
    println!("s2 is: {s2}"); // s2 is pointing to the same "Rust" s1 was pointing to
}

fn clone_ownership() {
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // s2 points to a new object that is like the one s1 is pointing to
    // s1 points to a "Rust" string on the heap
    // s2 points to another "Rust" string on the heap

    println!("s1 is: {s1}");
    println!("s2 is: {s2}");
}

fn primitives_ownership() {
    let x = 10;
    let y = x;

    // there is no move on primitives as they are stored on stack so each value is different because they are cloned by default
    println!("x is: {x}");
    println!("y is: {y}");
}

fn function_ownership() {
    let s1 = String::from("Rust");

    //print_string1(s1); // ownership of s1 is moved to the functions
    //println!("s1 is: {}", s1); // moved error!!!

    print_string(s1.clone()); // new string is created from s1
    println!("s1 is: {}", s1); // ownership is still valid here

    let s2 = generate_string(); // ownership of generated string is moved  to s2
    println!("s2 is: {}", s2); // ownership is okay here

    let s3 = add_to_string(s2); // ownership of s2 is moved to add_to_string1 
    //println!("s2 is: {}", s2); // error because s2 was moved to add_to_string
    println!("s3 is: {}", s3); // ownership is okay here

    let x = 10;
    print_integer(x); // x is a primitive so it is cloned because moving it to print_integer
    println!("x is: {}", x); // ownership is okay here

} // s1, s2, and s3 are dropped here

fn print_integer(i: i32) {
    println!("i is: {}", i);
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn generate_string() -> String{
    String::from("New string created")
}

fn add_to_string(mut p1: String) -> String { // p1 ownership is taken
    p1.push_str(" is awesome!"); // p1 must be mutable in order to use push_str
    p1 // and p1 ownership is returned
}