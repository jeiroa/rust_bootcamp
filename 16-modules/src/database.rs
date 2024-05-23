// this is module database (because of the name)
// no mod keyword needed
pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_database() -> Status {
    Status::Connected // it is a test so always return this
}

pub fn get_user() {
    // get user from database...
    println!("Getting user from DB...")
}