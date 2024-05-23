#![allow(dead_code, unused_variables)]
use std::fmt::Debug;

trait UIComponent: Debug {
    fn render(&self) {
        println!("Rendering component...");
    }
}

#[derive(Debug)]
struct Button {
    text: String,
}

impl UIComponent for Button {}

#[derive(Debug)]
struct Container {
    name: String, // String type so size is known
    //child: Container, // this is a recursive type which has infinite size so it does not compile
    child: Box<Container>, // now the type is Box which is known
}

impl UIComponent for Container {}

fn main() {
    // this button is stored on the stack
    let button_a = Button { text: String::from("Button_a") };
    // this button is stored on the heap
    let button_b = Box::new(Button { text: String::from("Button_b") });

    // Used to avoid copying large objects when transferring ownership

    // the entire Button will be copied to the new variable because it is on the stack
    let button_c = button_a; // ownership is transferred from button_a to button_c copying the Button object
    // only Box is copied because it is the one on the stack while the inner Button is still on the heap
    let button_d = button_b; // ownership is transferred from button_c to button_d copying the Box object but not the wrapped Button

    // Used to work with trait objects on the heap

    // if not type is set then only Box<Button> elements are allowed
    let components = vec![
        Box::new(button_c),
        Box::new(Button { text: String::from("Button_z") })
    ];
    println!("components: {:?}", components);

    // any UIComponent is now allowed using a trait object (remember it must be a pointer, so Box must me used)
    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(Button { text: String::from("Button_y") }),
        Box::new(Button { text: String::from("Button_z") })
    ];
    println!("components: {:?}", components);

    // this is the same as above but Button elements are stored on the stack and must be created in order to life enough to be printed below
    let button_y = &Button { text: String::from("Button_y") };
    let button_z = &Button { text: String::from("Button_z") };
    let components: Vec<&dyn UIComponent> = vec![
        button_y,
        button_z
    ];
    println!("components: {:?}", components);

    // Used to work with a type of unknown size in a context where the exact size is required, e.g. recursive types
    // see Container struct
}
