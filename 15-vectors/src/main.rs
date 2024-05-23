#![allow(dead_code, unused_variables)]

fn main() {
    // Vector are growable and are located in the heap
    // as they can grow, they must be mutable in order to add/remove elements
    // type of elements should be expecified of the type of the first element being added will be used
    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));
    // elements of a Vector are moved into it so Vector has ownership of its elements
    // this way, when the Vector is dropped, its elements are dropped as well

    // vector macro allows us to create Vectors as they were arrays
    let v2 = vec![1, 2, 3];

    // access elements by index 
    let s = &v[0]; // it can panic if size of v is 0

    // access elements by get method (safer)
    let s = v.get(0); // it does not panic because Option is returned in this case
    if let Some(elem) = s {
        println!("Element at 0: {elem}")
    }

    // iterate over the elements via for loop
    for s in &mut v { // v should be mutable because s will be updated and reference the vector to not to take ownership
        s.push_str("!"); // append an ! at the end of every element so s must be mutable
    }

    for s in &v { // here we do not need to define v as mutable
        println!("{s}"); // s will not change
    }

    // safely remove an element
    let s = v.remove(0); // it can also panic if size is 0 in this case
    println!("v: {:?}", v);

    // move elements from one vector to another one
    let mut v3 = vec![];
    for s in v { // this is actually calling v.into_iter() which moves self
        v3.push(s);
    }

    //println!("v: {:?}", v); --> a v.into_iter() moved self, v is not longer accesible
    println!("v3: {:?}", v3);
}
