#![allow(dead_code, unused_variables)]

// add an external dependency from crates.io
use rand::prelude::*; // access all components of module prelude

mod database; // modules defined as an external file of this project
mod auth_utils;

// auth_utils is a bit different because it contains a sub-module models
// in this case, a folder named auth_utils should be created and inside it a mod.rs should be created with
//  this context (login and logout functions and also models)
//  then, contents of models should be moved to a file models.rs in auth_utils folder and import to it
//  should be switched to pub mod models; to reference it
// + src
// |- auth_utils
//    |- mod.rs
//    |- models.rs
// |- lib.rs

// it is also possible to use a newer structure which is better:
// + src
// |- auth_utils
//    |- models.rs
// |- auth_utils.rs
// |- lib.rs
// auth_utils.rs contains the same code as mod.rs. It must be named same as the folder including the sub-module models

// pub must also be set to Credential use as it is used by authenticate func with is the public API function
pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = rand::thread_rng().gen_range(100..500); // use external dependency from rand

    println!("The timeout is {timeout}");

    let status = database::connect_database();
    if let Status::Connected = status {
        auth_utils::login(creds);
    }
}