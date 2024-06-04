use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration}; // thread module must be imported

// In this method a new thread might be created or not because the main thread might finish before it is created.
// It might happen that the thread is created but the loop does not complete because the main thread finishes before it.
fn unhandled_thread() {
    println!("---- Unhandled thread ----");

    // spawn function is used to execute a closure in a new thread
    thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread: {i}");
        }
    });

    for i in 0..10 {
        println!("Main thread: {i}");
    }
}

fn handled_thread() {
    println!("---- Handled thread ----");

    // get the join handle this time
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread: {i}");
        }
    });

    for i in 0..10 {
        println!("Main thread: {i}");
    }

    handle.join().unwrap(); // now the program will wait until the thread managed by the handle finishes

    // NOTE: execution of both threads seems sequencial.
    // This depends on the OS and it is more likely that it is due to the tasks are very simple and there is not change for the CPU for context switching.
}

fn handled_thread_sleep() {
    println!("---- Handled thread sleep ----");

    // get the join handle this time
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_millis(5)); // add a change for context switching
        }
    });

    for i in 0..10 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(10)); // add a change for context switching
    }

    handle.join().unwrap();
}

fn move_variable_into_thread() {
    let message = "Test message".to_owned();

    // it does not compile without move because the closure borrowed the message
    // and it might happen that the main thread lives longer than the spawn thread
    // so the spawned thread might be pointing to a dangled reference
    // this way, message must be moved into the closure so it takes ownership of it
    let handle = thread::spawn(move || {
        println!("{message}");
    });

    handle.join().unwrap();
}

fn message_passing() {
    let sentences = [
        "!dlroW wolleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!tsuR edoC s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned()
    ];

    let (tx, rx) = mpsc::channel();

    for s in sentences {
        // tx must be cloned because it is moved into the closure on every loop
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            // Err is returned if the receiver is already dropped
            tx_clone.send(reversed).unwrap();
        });
    }

    // dispose the channel sender because it is no longer necessary
    drop(tx); // otherwise the main thread will be blocked

    // this loop blocks every time next method is called waiting for a message
    for sentence in rx.iter() {
        println!("{sentence}");
    }
}

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Database {
        Database { connections: vec![] }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn use_mutual_exclusion_locks() {
    // use a mutex to safely share a resource
    let db = Mutex::new(Database::new());

    {
        // firstly it is necessary to get the mutex lock
        // thread is blocked until the lock is released
        let mut db_lock = db.lock().unwrap(); // a LockResult is returned

        // now it is possible to safely use the shared resource throught the lock variable
        db_lock.connect(1);
    } // lock is automaically released when it goes out of scope (MutexGuard implements Drop trait)
}

// share a database connection with all interested threads
fn share_state_between_threads() {
    // Mutex must be within a smart pointer in order to share the same instance in the closure below
    // if Rc is not used, the closure will complain about db to be moved more than once into it due to it is defined
    // within a for loop (the variable will be moved into several closures which is not allowed)
    // NOTE: Rc smart pointer cannot be used because it is not thread-safe so Arc is used instead
    let db = Arc::new(Mutex::new(Database::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let db = Arc::clone(&db); // now it is possible to clone the smart pointer so a new one is moved into the closure every time but containing the same Mutex

        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();

            db_lock.connect(i);
        }); // lock is released here

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let db_lock = db.lock().unwrap();

    println!("{db_lock:?}"); // print the content of the lock which is a Database instance
    // NOTE connections will be pushed in a non-deterministic order
}

fn main() {
    unhandled_thread();

    handled_thread();

    handled_thread_sleep();

    move_variable_into_thread();

    message_passing();

    use_mutual_exclusion_locks();

    share_state_between_threads();
}
