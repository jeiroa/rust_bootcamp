use crate::database;

pub fn login(creds: models::Credentials) {
    // authenticate

    database::get_user();
}

fn logout() {
    // log user out
}

pub mod models; // external submodule