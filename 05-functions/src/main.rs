fn main() {
    // calling a function
    my_function(22);
    // call a function and store the result
    let z = my_return_function(4);
    println!("z: {z}");
}

// fn + name (snake case) [+ param/s] [+ -> return type]
fn my_function(x: i32) {
    println!("my_fuction called with: {}", x);
}

// function that returns an i32
fn my_return_function(x: i32) -> bool {
    println!("my_return_fuction called with: {}", x);

    x > 5 // expressions at the end are the ones that contain the return value (no semicolon at the end)
    // return keyword can also be used but it must be ended by a ;
}
