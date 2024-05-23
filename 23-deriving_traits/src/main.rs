#[derive(Debug, PartialEq)] // enable derives to provides basic implementation of traits
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    // printing in debug format will not work unless the structure implements the Debug trait
    // deriving Debug is an alternative
    println!("{:?}", p1);

    // comparison does not work either because trait PartialEq is not implemented by Point
    // there is another PartialEq derive
    println!("{}", p1 == p2);
    println!("{}", p1 == p3);
}
