// import code of proc_macro (provided by the compiler) to build a procedural macro
// a procedural macro takes token string as input and returns token string as output
extern crate proc_macro;

use chrono::Utc;
use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, parse_quote, FnArg, Ident, ItemFn, Pat, Stmt};

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

// representation of the arguments of the attribe macro below
#[derive(FromMeta)]
struct LogCallMacroArgs {
    #[darling(default)] // this field is optional and will default to its default value, false in this case
    verbose: bool,
}

fn extract_arg_names(func: &ItemFn) -> Vec<&Ident> {
    func.sig.inputs.iter().filter_map(|arg| {
        // if arg is actually a method arg
        if let FnArg::Typed(pat_type) = arg {
            // and it is the arg name
            if let Pat::Ident(pat) = &(*pat_type.pat) {
                return Some(&pat.ident); // return the name
            }
        }
        None
    }).collect() // return the list with all idents
}

fn generate_verbose_log(fn_name: &Ident, fn_args: Vec<&Ident>) -> Vec<Stmt> {
    // firstly set the method name
    let mut statements = vec![parse_quote! {
        print!("[Info] calling {} | ", stringify!(#fn_name));
    }];
    
    // then set all arguments (name and value)
    for arg in fn_args {
        statements.push(
            parse_quote! {
                print!("{} = {:?} ", stringify!(#arg), #arg);
            }
        );
    }

    // and finally append a carry return
    statements.push(parse_quote! { println!(); });

    statements
}

// this method modifies the input to append code that prints information about the signature of the annotated method
fn impl_log_call(attr_args: &LogCallMacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident; // name of the function that was annotated

    if attr_args.verbose {
        // extract argument names
        let fn_args = extract_arg_names(input);
        // generate verbose log message
        let statements = generate_verbose_log(fn_name, fn_args);
        // prepend verbose log message to function block
        input.block.stmts.splice(0..0, statements); // append statements to position 0 of block statements (effectively prepend new statements)
    } else {
        input.block.stmts.insert(0, parse_quote! {
            println!("[Info] calling {}", stringify!(#fn_name));
        });
    }

    input.to_token_stream().into()
}

/// Attribute macros take 2 arguments: args with the arguments passed to the attribute and input with the tree of the annotated item
// like function-like macros, returned output token stream will replace the macro invocation at compile time
#[proc_macro_attribute] // attribute macro
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse function arguments to friendly structures
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(meta_args) => meta_args,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };

    let attr_args = match LogCallMacroArgs::from_list(&attr_args) {
        Ok(parse_args) => parse_args,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let mut input = parse_macro_input!(input as ItemFn);
    
    impl_log_call(&attr_args, &mut input)
}