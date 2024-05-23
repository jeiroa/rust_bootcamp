#![allow(dead_code, unused_variables)]
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

// implement Deref for MySmartPointer
impl<T> Deref for MySmartPointer<T>  {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// implement DerefMut for MySmartPointer
impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn print(s: &str) {
    println!("{}", s);
}

fn main() {
    let s = Box::new(String::from("Test"));

    // this is working because of implicit deref coercion (casting from &Box<String> to &str)
    print(&s); // &Box<String> --> &String --> &str

    // Deref coercion works on types implementing Deref and DerefMut traits

    let s2 = MySmartPointer::new(Box::new(String::from("test")));
    // it works because MySmartPointer<Box<String>> coerce to Box<String> as it implements Deref trait
    print(&s2);  // &MySmartPointer<Box<String>> --> &Box<String>> --> &String --> &str
    // below are the deferrences coerced which are resolved at compile time so there is no performance cost at runtime
    let s3 = &(*s2);
    let s3 = &(**s2);
    let s3 = &(***s2);

    let mut s4 = MySmartPointer::new(Box::new(String::from("test")));
    // both calls are valid because the method expects &str
    print(&s4); // it would not work if the method expects &mut str
    print(&mut s4);
}
