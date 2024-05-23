#![allow(dead_code, unused_variables)]

// 02 Data Types. See https://doc.rust-lang.org/rust-by-example/primitives.html
fn main() {
    // scalar data types (they store a single value)
    // boolean
    let b1: bool = true;

    // unsigned integers
    let u1: u8 = 1;
    let u2: u16 = 1;
    let u3: u32 = 1;
    let u4: u64 = 1;
    let u5: u128 = 1;

    // signed integers
    let i1: i8 = -1;
    let i2: i16 = -1;
    let i3: i32 = -1;
    let i4: i64 = -1;
    let i5: i128 = -1;

    // floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1; // unsigned int of 4 bytes in 32 bits systems or 8 bytes in 64 bits ones
    let p2: isize = -1; // signed int of 4 bytes in 32 bits systems or 8 bytes in 64 bits ones

    // chars and strings
    let c1: char = 'c'; // unicode scalars of 4 bytes, between single quotes
    let s1: &str = "hello"; // default string type
    let s2: String = String::from("hello");

    // compound data types
    // arrays (multiple values of the same type)
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [1, 2, 3];
    let a3: i32 = a1[4]; // assign position 4 os a1 to a3 integer scalar

    // typles (multiple values of different types)
    let t1 = (1, 2, 3);
    let t2 = (1, 2.0, "3");

    let s1 = t2.2; // assign value of position 2 of t2 ("3")
    let (i1, f1, s1) = t2; // destructure t2 into 3 variables

    println!("i1: {:?}; f1: {:?}; s1: {:?}", i1, f1, s1);

    let unit= (); // unit type is the empty tuple, used to return a value when no meaningful value can be returned, e.g. functions that do not return a value

    // type aliasing (to define new names for existing types just to make the code easier to understand)
    type Age = u8; // age type that is actually an unsigned 8 bits integer
    let a1: Age = 67;
}
