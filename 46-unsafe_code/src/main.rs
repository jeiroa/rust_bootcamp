use std::arch::asm;

// useful to achieve greater performance, interface with other languages and hardware
fn dereference_raw_pointer() {
    let mut s = String::from("Pointer content");

    // these variables contain the address where the string is
    // they are valid addresses because they are cast from a valid String variable
    let raw1 = &s as *const String; // immutable raw pointer
    let raw2 = &mut s as *mut String; // mutable raw pointer

    println!("{:?}", raw1);
    println!("{:?}", raw2);

    // the following immutable raw pointer is problably pointing to invalid memory
    let address = 0x012345usize;
    let raw3 = address as *const String;

    println!("{:?}", raw3); // reference and access raw pointers is safe


    unsafe {
        (*raw2).push_str("!!!"); // dereference a raw pointer is unsafe and must be within a unsafe block
        println!("raw1 is: {}", *raw1);

        //println!("raw3 is: {}", *raw3); // memory violation error!!!
    }
}

// function where some invariants must be taking into account by the dev
unsafe fn my_function() {
    println!("Calling unsafe function");
}

fn call_unsafe_function() {
    unsafe {
        my_function();
    }
}

// some invariant must be implemented by the dev
unsafe trait MyTrait {
    unsafe fn some_function(&self);
}

unsafe impl MyTrait for String {
    unsafe fn some_function(&self) {
        // ...
    }
}

fn implement_unsafe_trait() {
    let s = String::from("some string");
    unsafe {
        s.some_function();
    }
}

static mut COUNTER: u32 = 0;

fn increment_counter() {
    // this is because two threads modifying a mutable static var at the same time might lead to a data race condition
    unsafe {
        COUNTER += 1;
    }
}

fn mutable_static_variable() {
    for _ in 0..10 {
        increment_counter();
    }

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

fn sum(x: u64, y:u64) -> u64 {
    let result: u64;

    unsafe {
        // x86/x86-64 assembly
        asm!("add {0}, {1}", inout(reg) x => result, in(reg) y);
    }

    result
}

fn inline_assembly() {
    println!("sum: {}", sum(2, 3));
}

#[link(name = "adder", kind = "static")]
extern "C" {
    fn add(a: i32, b:i32) -> i32;
}


/// execute with RUSTFLAGS='-L ./adder' cargo run
fn external_code() {
    let x:i32;

    unsafe {
        x = add(1, 2);
    }

    println!("x is: {x}");
}

fn main() {
    // 1. Dereference a raw pointer
    dereference_raw_pointer();
    // 2. Call an unsafe function
    call_unsafe_function();
    // 3. Implement an unsafe trait
    implement_unsafe_trait();
    // 4. Access/Modify a mutable static variable
    mutable_static_variable();
    // 5. inline assembly
    inline_assembly();
    //6. Call external code via FFI (Foreign Function Inteface)
    external_code();
}
