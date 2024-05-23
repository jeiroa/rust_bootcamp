// generic lifetime annotations are necessary when storing references in struts
struct Tweet<'a> {
    content: &'a str,
}

// Elision rules (if any is met, no lifetime annotation is necessary because there is no ambiguity)
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime
//    is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all ouputs
//    lifetime parameters.
// Examples:
// fn print(s: &str);                                      // elided
// fn print<'a>(s: &'a str);                               // expanded
//
// fn debug(lvl: usize, s: &str);                          // elided
// fn debug<'a>(lvl: usize, s: &'a str);                   // expanded
//
// fn substr(s: &str, until: usize) -> &str;               // elided
// fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded
//
// fn get_str() -> &str;                                   // ILLEGAL
//
// fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL
//
// fn get_mut(&mut self) -> &mut T;                        // elided
// fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded
//
// fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command                  // elided
// fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command // expanded
//
// fn new(buf: &mut [u8]) -> BufWriter;                    // elided
// fn new(buf: &mut [u8]) -> BufWriter<'_>;                // elided (with `rust_2018_idioms`)
// fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>          // expanded

// generic lifetime annotations are also needed when implementing the struct
impl<'a> Tweet<'a> {
    // content param is &'a str because of the 3rd ellision rule (return is of &'a str type)
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

fn main() {
    // tweet should be mutable because it is updated in the implementation (see self param)
    let mut tweet = Tweet {
        content: "test"
    };
    let old_content = tweet.replace_content("example");
    println!("old: {}", old_content);
    println!("current: {}", tweet.content);
}

fn take_and_return_content(content: &str) -> &str {
    content // there is no need to set a lifetime annotation based on the 3rd elision rule
}

// output param (return) is of type &'a str
fn take_and_return_content2<'a>(content: &'a str) -> &str {
    content // there is no need to set a lifetime annotation based on the 2nd elision rule
}

// output needs a lifetime notation because there are two input parameters (does not meet rule 2)
fn take_and_return_content3<'a, 'b>(content: &'a str, content2: &'b str) -> &'a str {
    content
}