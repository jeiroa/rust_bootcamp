#![allow(dead_code, unused_variables)]
struct Credentials {
    username: String,
    password: String,
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

// this version contains a field that is a closure
// closure types should be defined like generics whose type is
// - Fn: Immutably borrow variables in environment.
// - FnMut: Mutably borroe variables in environment. Can change environment.
// - FnOnce: Take ownership of variables in environment. Can only be called once.
struct Credentials2<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T,
}

// an implementation with a closure must be added
impl<T> Credentials2<T> where T: Fn(&str, &str) -> bool {
    // this is the function to be called to use the closure
    fn is_valid(&self) -> bool {
        // closure field must be between (). Same for its arguments.
        (self.validator)(&self.username, &self.password)
    }
}

// function that accepts a closure (&str, &str) -> bool as a parameter
fn get_default_creds<T>(f: T) -> Credentials2<T> where T:Fn(&str, &str) -> bool {
    Credentials2 {
        username: "guest".to_owned(),
        password: "password123!".to_owned(),
        validator: f
    }
}

fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    // if move is not used then min_len does not live longer because it is a method parameter
    // it is not possible to return directly two different implementations of a closure even when both implement
    // the same signature. This is why Box pointer has to be used.
    if special_char {
        Box::new(move |_:&str, password: &str| {
            !password.len() >= min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        })
    } else {
        Box::new(move |_:&str, password: &str| !password.len() >= min_len)
    }
} // min_len is dropped here

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
    where T:Fn(&V) -> bool, U: Fn(&V) -> bool{
    // f1 and f2 are validators of item value parameter
    f1(item) && f2(item)
}

fn less_than_func(x: &i32) -> bool {
    *x < 20
}

// same function but accepting function methods instead of closures
// notice that fn is defined with its paramters between () and the return type if any
// fn stands for function while Fn stands for closure
// as functions are concrete types, no generics are necessary
// but it is better to use closure parameters as they allow both closures and function pointers
fn are_both_true_fn<V>(f1: fn(&V) -> bool, f2: fn(&V) -> bool, item: &V) -> bool {
    // f1 and f2 are validators of item value parameter
    f1(item) && f2(item)
}

fn main() {
    let creds = Credentials {
        username: "admin".to_owned(),
        password: "pass".to_owned()
    };

    println!("{}", validate_credentials(&creds.username, &creds.password));

    // closure like validate_credentials method
    // |argument: type, ...| -> return type { body }
    // type/s and return type can be removed if Rust is able to infer it/them
    // {} are optional if body is just one line
    let validator = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty()
    };

    // just call the closure via its variable
    println!("{}", validator(&creds.username, &creds.password));

    let creds2 = Credentials2 {
        username: "admin".to_owned(),
        password: "pass".to_owned(),
        validator: validator
    };

    println!("{}", creds2.is_valid());

    let weak_password = "password123!".to_owned();

    let validator2 = |username: &str, password: &str| {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != weak_password // variables in the same context can be used in closures
    };

    println!("{weak_password}"); // closures do not take ownership of external vars by default (move should be used instead)

    // provide a new validator
    let creds3 = Credentials2 {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2
    };

    println!("{}", creds3.is_valid());

    let password_validator = get_password_validator(8, true);
    // Boxed closures can be passed to closure params because they implement Fn, FnMut, and FnOnce
    let default_creds = get_default_creds(password_validator);

    // function pointers can also be used
    let greather_than = |x: &i32| *x > 10;
    let less_than = |x: &i32| *x < 20;
    // call the function with two closures
    let result = are_both_true(greather_than, less_than, &9);
    println!("are_both_true: {result}");
    // call the same function with one closure and one function pointer
    // regular functions can be passed to closure parameter types
    let result = are_both_true(greather_than, less_than_func, &9);
    println!("are_both_true: {result}");

    let ten = 10;
    // use a variable from the closure (this cannot be done with pointer functions)
    let greater_than_var = |x: &i32| *x > ten;
    let result = are_both_true(greater_than_var, less_than_func, &9);
    println!("are_both_true: {result}");
}
