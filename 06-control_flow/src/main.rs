fn main() {
    // if/else
    let a = 5;

    // parantheses are unnecessary
    if a > 5 {
        println!("{a} is bigger than 5");
    } else if a > 3 {
        println!("{a} is bigger than 3 and smaller or equal to 5");
    } else {
        println!("{a} is smaller or equal to 3");
    }

    // terciary operation
    let _b = if a > 5 { 1 } else { -1 };

    // loop loops forever until a break is found.
    let mut i = 0;
    loop {
        if i == 5 {
            break;
        } else {
            i = i + 1;
        }
    }

    // loops can also return values
    let x = loop {
        break 8;
    };

    // x is 8
    println!("x is: {x}");

    // while loop is the usual one
    i = 0;
    while i < 5 {
        println!("i in while: {i}");
        i = i + 1;
    }

    // for loop
    let a = [1, 2, 3, 4, 5];

    // iterate through a collection
    for element in a {
        println!("element in for: {element}")
    }

    // exclusive range (1, 2, 3, 4)
    for n in 1..5 {
        println!("n in for: {n}")
    }
    // inclusive range (1, 2, 3, 4, 5)
    for n in 1..=5 {
        println!("n in for: {n}")
    }
}
