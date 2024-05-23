#![allow(dead_code, unused_variables)]

fn main() {
    // tuples (group together data of different types but no type is defined)
    // only values between () and comma separated are necessary
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs (it is a tuple defined as a new type as a struct)
    // struct + <name> + ( + types + )
    #[derive(Debug)]
    struct RGB(i32, i32, i32);
    #[derive(Debug)]
    struct CMYK(i32, i32, i32, i32);

    // now tuples are typed and all fields need to be provided with the proper type
    let color1 = RGB(255, 106, 0);
    let color2 = CMYK(0, 58, 100, 0);

    println!("{:?}", color1);
    println!("{:?}", color2);

    // unit-like structs (structs with no fields)
    // they are very rarely used (to implement a trait where it is not needed to store any data in it)
    struct MyStruct;
}
