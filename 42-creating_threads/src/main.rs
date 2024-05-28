use std::{thread, time::Duration}; // thread module must be imported

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

fn main() {
    unhandled_thread();

    handled_thread();

    handled_thread_sleep();
}
