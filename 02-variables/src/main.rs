#![allow(dead_code, unused_variables, unused_assignments)]

fn main() {
    // creation (let, variable name, :optional type, = , initial value)
    // type adjusts to the initial value type (i32 for integers)
    let a = 2;
    // f64 for decimal
    let b = 7.0;
    // set the type
    let c: u8 = 5;
    //let c: u8 = 5.0; <-- error, mismatched types

    // mutability
    // let m = 5;
    // m = 10; <-- error because variables are immutable by default so they cannot be modified
    let mut m = 5; // define variable as mutable
    m = 10; // now it can be modified

    // shadowing
    let s = 10;
    let s = 20; // s is shadowing the previous value

    println!("s is: {s}"); // s is 20

    // scope
    let d = 30; // scope is the main function (brackes define scope)

    { // new scope
        let d = 40; // this d shadows the previous one in this scope

        println!("inner d is: {}", d); // <-- inner d is: 40
    } // end of scope

    println!("d is: {}", d); // <-- d is: 30

}
