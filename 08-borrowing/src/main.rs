fn main() {
    let s1 = String::from("Rust");
    let mut s2 = String::from("Rust");
    let r1 = &s1; // immutable reference to s1
    let r2 = &mut s2; // mutable reference to s2 (cannot make mutable reference to s1 because it is immutable)

    print_string(r1); // borrow s1 to print_string
    println!("s1 is: {}", s1); // s1 is okay here because of borrowing

    add_to_string(r2); // borrow r2 which is a reference to s2
    println!("r2 is: {}", r2); // r2 is okay here because of borrowing and it is also modified by reference because it points t s2
    println!("s2 is: {}", s2); // s2 is okay here because of borrowing and it is also modified by reference

    let s3 = generate_string();
    println!("s3 is: {}", s3);
}

// references cannot be returned because inner variables are dropped at the end of the function
//fn generate_string() -> &String { <-- error here
//    let s = String::from("Created String");
//    &s // dangling reference
//} // s is dropped here!!!

fn generate_string() -> String {
    String::from("Created String") // do not try to return references
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome!"); // p1 is modified by reference
    // Rust uses automatic derefencing
    //(*p1).push_str(" is awesome!"); <-- this is manual dereferencing
}

fn print_string(p1: &String) { // & is used so p1 value is borrowed
    println!("p1 is {}", p1);
} // return borrowed value p1