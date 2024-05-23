#![allow(dead_code, unused_variables)]
use std::rc::Rc; // Rc allows shared ownership of components
use std::cell::RefCell; // RefCell allows to update its reference

#[derive(Debug)]
struct Database {
    max_connections: u16,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database { max_connections: 100 } ));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    db.borrow_mut().max_connections = 200; // mutably borrows the value wrapped in the RefCell, even when it is not originally mutable

    // WARNING: there are situations where ownership is not detected by the compiler when using RefCell

    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut(); // there is no compiler error but program wil panic because there is already a mutable variable of db

    r1.max_connections = 50;
    println!("{:?}", r2);
}