#![allow(dead_code, unused_variables)]
use std::rc::Rc; // Rc allows shared ownership of components

struct Database {
    max_connections: u16,
}

struct AuthService {
    db: Database,
}

struct ContentService {
    db: Database,
}

struct RcAuthService {
    db: Rc<Database>,
}

struct RcContentService {
    db: Rc<Database>,
}

fn main() {
    // shared database object
    let mut db = Database { max_connections: 100 };
    let auth_service = AuthService { db: db };
    let content_service = ContentService { db: db }; // error because db ownership was already moved

    // shared object is now of type Rc (Reference-Counting pointer)
    let rc_db = Rc::new(Database { max_connections: 100 }); // RC counter is created with value 1
    let rc_auth_service = RcAuthService { db: Rc::clone(&rc_db) }; // RC counter is increased here (2)
    let rc_content_service = RcContentService { db: Rc::clone(&rc_db) }; // RC counter is increased here (3)

    // NOTE: RC pointers can also be used in single-threaded applications

    //rc_db.max_connections = 200; error because Rc only allows immutable ownership of its value RefCell should be used instead
}
// RcContentService, RcAuthService are dropped here, so their Rc dependecies are dropped too an decremented to 1
// finally, the database object is also dropped, the Rc is decreased to 0 and it is finally dropped
