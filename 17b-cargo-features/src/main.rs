#![allow(dead_code, unused_variables)]

use cargo_features_lib::color; // import color module available in default, color, and shapes feature
use cargo_features_lib::shapes; // shapes module is available only if shapes features is enabled

fn main() {
    cargo_features_lib::draw_line(32, 32);

    let color = color::RGB {
        r: 247,
        g: 76,
        b: 0,
    };

    color::draw_line(32, 32, &color);

    let rectangle = shapes::Rectangle {
        color,
        width: 32,
        height: 32,
    };
}
