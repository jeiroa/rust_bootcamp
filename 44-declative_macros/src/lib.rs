// library where all custom macros to use are created

/// macro_rules! <name> {
///     rule0;
///     rule1;
///     ...
///     ruleN;
/// }
/// 
/// rules:
/// (matcher) => { expansion aka transcriber }
/// 
// new declarative macro called hello
#[macro_export] // annotation necessary to export the macro
macro_rules! hello {
    // rule0
    () => {
        // body of the macro
        println!("Hello World!");
    };
}

#[macro_export]
macro_rules! map {
    // these metavariables follow the format $[identifier] : [fragment-specifier]
    // See https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
    // $key and $val contain the types of the key and the value of the HashMap
    ($key:ty, $val:ty) => {
        { // scope necessary to return the map variable
            let map: HashMap<$key, $val> = HashMap::new();
            map
        }
    };

    // rule1 includes param repetition in the form of $(param, times)
    // times can be * (0 or more), ? (0 or 1), + (at least 1)
    ($($key:expr => $val:expr), *) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $val); )* // this means to repeat for all arguments
            map
        }
    }
}