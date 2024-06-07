// import code of proc_macro (provided by the compiler) to build a procedural macro
// a procedural macro takes token string as input and returns token string as output
extern crate proc_macro;

use chrono::Utc;
use proc_macro::TokenStream;
use quote::quote;
use syn;

/// this macro just returns the code passed as input.
/// If [TIME] token is fount, then it is replaced by the current UTC time.
// returned output token stream will replace the macro invocation at compile time
#[proc_macro] // function-like macro
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info] ".to_owned();

    // process each token in the input
    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            // for this one, return the current UTC time
            "[TIME]" => {
                let time = Utc::now().time().to_string();
                output.push_str(&format!("{} ", time));
            }
            // for the rest item just return them as they come
            _ => {
                output.push_str(&format!("{} ", token_string));
            }
        }
    }

    // return the output string correctly created
    // TokenStream::from converts the macro output from quote! (proc_macro2::TokenStream) to a proper TokenStrem (proc_macro::TokenStream)
    TokenStream::from(quote! { // convert the following source code into a token output
        println!("{}", #output);
    })
}

/// this macro can be used to provide a default implementation:
/// #[derive(Log)]
// returned output will not replace the macro invocation but it will be appended to the containing block or module of the annotated item
#[proc_macro_derive(Log)] // derive macro. Log is the identifier to be set in the derive attribute
pub fn log_derive(input: TokenStream) -> TokenStream {
    // parse input into a derive input struct
    let ast: syn::DeriveInput = syn::parse(input).unwrap(); // DeriveInput represents items in #[derive()]
    
    let name = ast.ident;

    let trait_impl = quote! { // transform Rust code into a TokenStream
        impl Log for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {}: {}", stringify!(#name), msg);
            }

            fn warn(&self, msg: &str) {
                println!("[Warn {}: {}", stringify!(#name), msg);
            }

            fn error(&self, msg: &str) {
                println!("[Error] {}: {}", stringify!(#name), msg);
            }
        }
    };

    trait_impl.into() // same as TokenStream::from
}