#![allow(dead_code, unused_variables)]

// Traits a re used to share functionality and provide a common interface
// they are similar to Java interfaces
trait Park {
    // trait methods usually do not provide an implementation
    fn park(&self);
}

trait Paint {
    // but one default implementation may be provided
    fn paint(&self, color: String) {
        println!("Painting object: {}", color);
    }
}

// Vehicle also includes Paint and Park traits (is like extending interfaces in Java)
// it is possible to include more than one trait (use + to separate them)
// Paint and Park are supertraits
trait Vehicle: Paint + Park {
    fn drive(&self);
    // associated methods (static) can also be available in traits
    // notice that no &self parameter is specified
    fn get_default_color() -> String {
        String::from("white")
    }
}

// common Car and Truck data
struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("Unloading truck.")
    }
}

struct House {}

// implementation of Park trait for Car
impl Park for Car {
    fn park(&self) {
        println!("Parking car!")
    }
}

// override default paint method in case of cars
impl Paint for Car {
    fn paint(&self, color: String) {
        println!("Painting car: {}", color);
    }
}

// implementation of Park trait for Truck
impl Park for Truck {
    fn park(&self) {
        println!("Parking truck!")
    }
}

// default implementation of paint is used here
impl Paint for Truck {}

// houses can also be painted
impl Paint for House {}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: String::from("Honda"),
            model: String::from("Civic"),
            year: 1996
        }
    };

    let house = House {};
    let object = create_paintable_object();

    // it compiles because Car and House implements P and object is an actual Paint
    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    // ony Car implements Paint and Park traits
    paint_park_vehicle_red(&car);
    //paint_park_vehicle_red(&house); // House does not implement Paint
    //paint_park_vehicle_red(&object); // does not implement Park as it is an actual Paint

    // use dynamic trait objects
    let obj = create_paintable_object_bool(true);
    paint_red_dynamic(obj.as_ref());

    // trait objects are also used in collections of generic elements
    //let paintable_objects: Vec<Paint> = vec![car, house]; // it does not work because it is expecting a dynamic object trait type
    //let paintable_objects: Vec<dyn Paint> = vec![car, house]; it does not work because trait objects must be referenced somehow
    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

}

// this method does not compile because it is unkwon if T implements a paint method or not
//fn paint_red<T>(object: &T) {
//    object.paint(String::from("red"));
//}

// Three ways to define that T must implement Paint trait
fn paint_red<T: Paint>(object: &T) {
    object.paint(String::from("red"));
}

fn paint_red2(object: &impl Paint) {
    object.paint(String::from("red"));
}

fn paint_red3<T>(object: &T) where T: Paint {
    object.paint(String::from("red"));
}

// the third method can be used to state that T should implement two or more traits
fn paint_park_vehicle_red<T>(object: &T) where T: Paint + Park {
    object.paint(String::from("red"));
    object.park();
}

// add impl to return type (a trait) to state that something implenting that trait will be returned
fn create_paintable_object() -> impl Paint {
    House {}
}

// rewrite previous method to return different implentations depending on a parameter
// this fails because the generic type must be substituted by a concrete type at compile time (static dispatch) and it is unknown in the case of an if/else
// to fix this error is necessary to return a trait object which is dynamic and must be defined within any pointer
//fn create_paintable_object_bool(vehicle: bool) -> impl Paint {
//    if vehicle {
//        Car {
//            info: VehicleInfo {
//                make: String::from("Toyota"),
//                model: String::from("CHR"),
//               year: 2023
//            }
//        };
//    } else {
//        House {}
//    }
//}

// dynamic dispatch trait objects allows to return generic implementations that are unknown at compile time inserting code able to figure it out at runtime (at a performance cost).
// a Box smart pointer is used to wrap the dynamic trait object
fn create_paintable_object_bool(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box:: new(Car {
            info: VehicleInfo {
            make: String::from("Toyota"),
            model: String::from("CHR"),
            year: 2023
            }
        })
    } else {
        Box::new(House {})
    }
}

// this version of paint_red method exposes a dynamic trait object
fn paint_red_dynamic(object: &dyn Paint) {
    object.paint(String::from("red"));
}

fn create_vehicle<T>(object: &T) where T: Vehicle {
    object.drive();
    object.park();
    object.paint(String::from("black"));
    T::get_default_color();
}