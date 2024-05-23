#![allow(dead_code, unused_variables)]

pub fn draw_line(x: i32, y: i32) {
    // draw line without color
}

#[cfg(feature = "color")] // include this module only if color feature is enabled
pub mod color {
    pub use rgb::RGB; // this is the only dependency in this case so executable size will be smaller
    
    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("{color}")
        // draw line with color
    }
}

#[cfg(feature = "shapes")] // include this module only if shapes feature is enabled
pub mod shapes {
    use serde::{Serialize, Deserialize};
    use rgb::RGB;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<u16>,
        pub width: u32,
        pub height: u32,
    } 
}