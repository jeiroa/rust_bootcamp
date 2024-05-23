#![allow(dead_code, unused_variables)]

// constants (const + NAME: type = value)
const MAX_PLAYERS: u8 = 10; // type is mandatory
// static variables (static + NAME: type = value)
static CASINO_NAME: &str = "Rusty Casino";


fn main() {
    // consts and statics can be declared in any scope
    const OTHER_PLAYERS: u8 = 12;
    // static variables can be mutable
    static mut CASINO_NAME_MUTABLE: &str = "Rusty Casino II";
    // but to modify them it is necessary to be in an unsafe block
    unsafe {
        CASINO_NAME_MUTABLE = "Rusty Casino III";
    }

    // constants do not occupy specific location in memory as its value is inlined
    let players = MAX_PLAYERS; // this is compiled as let players: u8 = 10;
    // static vars do occupy memory
    let name = CASINO_NAME; // this is compiled as it is
}
