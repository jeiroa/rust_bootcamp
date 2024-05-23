use orphan_rule::Point;

// it does not compile because this implentation is breaking the orphan rule
// which states that in order to implement a trit for a type either the trait (PartialEq) or the type (Point) must be defined within the current crate
// in this case, PartialEq is defined in the core crate and point is defined in the project's library crate
//impl PartialEq for Point {
//    fn eq(&self, other: &Self) -> bool {
//        self.x == other.x && self.y == other.y
//    }
//}

// In order to fix the orphan rule issue it is necessary to create a wrapper for the type and provide the trait implementation for it
struct PointWrapper(Point);

// now the wrapper type is in this crate so the implementation is valid
impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn main() {
    let p1 = PointWrapper(Point { x: 1, y: 2 });
    let p2 = PointWrapper(Point { x: 1, y: 2 });

    println!("{}", p1 == p2);
}
