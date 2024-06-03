use std::{sync::mpsc, thread, time::Duration}; // thread module must be imported

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

fn main() {
    unhandled_thread();

    handled_thread();

    handled_thread_sleep();

    move_variable_into_thread();

    message_passing();
}
