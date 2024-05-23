#![allow(dead_code, unused_variables)]

fn main() {
    concrete_lifetimes();
    generic_lifetimes();
}

fn concrete_lifetimes() {
    concrete_lifetimes_example1();
    concrete_lifetimes_example2();
    concrete_lifetimes_example3();
    concrete_lifetimes_example4();
    concrete_lifetimes_example5();
}

fn generic_lifetimes() {
    generic_lifetimes_example1();
    generic_lifetimes_example2();
    generic_lifetimes_example3();
}

fn concrete_lifetimes_example1() {
    let s1 = String::from("test"); // s1 lifetime starts here

    println!("s1: {s1}"); // s1 is valid here
} // s1 lifetime ends here

fn concrete_lifetimes_example2() {
    {
        let s1 = String::from("test"); // s1 lifetime starts here
    } // s1 lifetime ends here

    println!("s1: {s1}"); // s1 is not valid here
}

fn concrete_lifetimes_example3() {
    let s1 = String::from("test"); // s1 lifetime starts here
    println!("s1: {s1}"); // s1 is valid here
    let s2 = s1; // s2 lifetime starts here and s1 lifetime ends here (s1 is moved to s2)
    println!("s1: {s1}"); // s1 is not valid here
}

fn concrete_lifetimes_example4() {
    let r1; // r1 lifetime starts here

    {
        let s1 = String::from("test"); // s1 lifetime starts here
        r1 = &s1; //error indicating that s1 is not valid from the next line so r1 would be a dangled reference
    } // s1 lifetime ends here

    println!("r1: {r1}"); // r1 is valid here but s1, at which r1 is pointing to, is not
}

fn concrete_lifetimes_example5() {
    let mut s1 = String::from("test"); // s1 lifetime starts here
    let r1 = &s1; // r1 lifetime starts here
    println!("r1: {r1}");
    // from here it seems we are breaking the borrowing rule where at a given time there should only be either one mutable reference or multimple immutable references.
    // there is an immutable reference at r1 = &s1 and a mutable one below
    let r2 = &mut s1; // r2 lifetime starts here
    r2.push_str("!");
    // the compiler realizes that r1 is valid for 2 lines before r2 is declared and used so they are not actually living at the same time
}

fn generic_lifetimes_example1() {
    let player1 = String::from("Player 1");
    let player2 = String::from("Player 2");

    let result = first_turn_example1(player1.as_str(), player2.as_str());
    // lifetime of result is unknown 
    println!("Player going first is: {}", result);

    let result = first_turn_example2(player1.as_str(), player2.as_str());
    // this works because player1 and player2 are both in scope until the end of this method
    println!("Player going first is: {}", result);
}

fn generic_lifetimes_example2() {
    let player1 = String::from("Player 1");
    let result;
    {
        let player2 = String::from("Player 2");

        result = first_turn_example2(player1.as_str(), player2.as_str());
    }
    // the shortest lifetime is now the reference to player2 and as it is no valid after the previous line, the call to the method fails
    println!("Player going first is: {}", result);
}

fn generic_lifetimes_example3() {
    let player1 = String::from("Player 1");
    let player2 = String::from("Player 2");

    // this does not work because it is not possible to return a dangled reference
    let result = first_turn_example3a(player1.as_str(), player2.as_str());
    println!("Player going first is: {}", result);
    // works fine because a reference to player1 is always returned and it is in scope
    let result = first_turn_example3b(player1.as_str(), player2.as_str());
    println!("Player going first is: {}", result);
    // it works because the local reference returned has static lifetime
    let result = first_turn_example3c(player1.as_str(), player2.as_str());
    println!("Player going first is: {}", result);

}

// this method is returning one of its arguments as a reference but both are references so it does not know the exact lifetime of the value to be returned
// this way, Rust compiler fails in order to avoid dangled references
// in order to avoid this flivetime ambiguity it is necessary to use generic lifetime specifiers
fn first_turn_example1(p1: &str, p2: &str) -> &str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

// generic lifetime specifiers are declared with a ' before a label, normally a lowercase letter starting with a
// this label indicates that those references have a relationship where the lifetime returned will be the shortest lifetime of the references with that label
// in this case, the lifetime to be returned will be the shortest between p1 and p2.
// first_turn_example2 is called being p1 -> player1 and p2 -> player2 in generic_lifetimes_example1
// as they both are declared at the begining and disposed at the end of the method, the lifetime is the same and when the result is used, both are valid so everything will be okay
fn first_turn_example2<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

fn first_turn_example3a<'a>(p1: &'a str, p2: &str) -> &'a str {
    let s = String::from("test"); // s lifetime starts here

    s.as_str() // it is not possible to return a local reference via generic lifetime specifiers
} // s lifetime ends here

// reference returned is usually a reference to an input parameter in order to provided a proper lifetime
fn first_turn_example3b<'a>(p1: &'a str, p2: &str) -> &'a str {
    p1
}

// static lifetimes are valid for the entire duration of the program
// function signature is refactored as we are returning a function local reference with static lifetime
fn first_turn_example3c(p1: &str, p2: &str) -> &'static str {
    let s: &'static str = "test"; // a string slice is stored in the program's binary so it can has static lifetime associated with the program's life

    s // local string slice with static lifetime
}