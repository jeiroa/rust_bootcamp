use std::{thread, time::Duration};

use tokio::time::sleep;
use tokio_stream::StreamExt; // necessary for Tokio Streams

/// async keyword actually provides the following code:
/// 
/// fn my_function() -> impl std::future::Future<Output = ()> { // return type is () as the async function below defined
///     println!("I am an async function");
/// }
/// 
/// This function returns something that implements the Future trait that does not return anything in this case
/// 
/// Future trait is defined as:
/// trait Future {
///     // return type
///     type Output;
///     // it check if this instance is ready to return a value
///     // wake is a callback that notifies the executor when this future can be polled again
///     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
/// }
/// 
/// enum Poll<T> {
///     Ready(T), // future is ready to return a value of type T
///     Pending, // future is still pending
/// }
async fn my_function(i: i32) {
    println!("[{i}] I am an async function on {}-{:?}", thread::current().name().unwrap(), thread::current().id());
    // await attempts to run the future until completion
    // execution of caller Future is paused, yielding control back to the runtime
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1} on {}-{:?}", thread::current().name().unwrap(), thread::current().id());
    let s2 = read_from_database().await; // execution is paused here as well
    println!("[{i}] Second result: {s2} on {}-{:?}", thread::current().name().unwrap(), thread::current().id());
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}

/// Main is the initiator function so await cannot be used to execute async functions.
/// It is necessary to manage calls to the poll method and handle the wake callback. This is a task for a runtime.
/// The runtime will call the poll method until completion, handling the Future state processing the callback defined in the wake parameter.
/// This runtime will also be able to manage several Futures in parallel.
/// Rust does not provide a built-in runtime so a community one should be used, being Tokio the most popular as of now.
#[tokio::main] // use Tokio runtime to execute this function
async fn serial_tokio() {
    //my_function(); // this will not execute anything because Futures are lazy
    my_function(0).await; // this will not work until a runtime is used

    let future = my_function(0);
    println!("Execute before the future");
    future.await;
    println!("Future executed");

    // NOTE tokio::main can also be used in the main method
}

// default configuration is #[tokio::main(flavor = "multi_thread", worker_threads = # of system CPUs>)]
// #[tokio::main(flavor = "current_thread")] will use just one thread and time slicing
#[tokio::main]
async fn tokio_tasks() {
    let mut handles = vec![];

    for i in 0..5 {
        // tokio::spawn does not create a thread but a task that will be executed on a thread from Tokio's worker-thread pool
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

fn expensive_computation() {
    let mut _i = 0;

    println!("Expensive computation on {}-{:?}", thread::current().name().unwrap(), thread::current().id());

    for _ in 0..400_000_000 {
        _i += 1;
    }

    println!("Expensive computation done on {}-{:?}", thread::current().name().unwrap(), thread::current().id());
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn tokio_expensive_computation() {
    let mut handles = vec![];

    // prepare cheap computation tasks
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });

        handles.push(handle);
    }

    // as expensive computation tasks will block a thread longer than other tasks
    handles.push(tokio::spawn(async {
        // a spawn_blocking should be used so a separated working thread will be use to run this task
        // this way, working threads on the normal pool will not be blocked by this task
        let _res = tokio::task::spawn_blocking(|| {
            expensive_computation();
        }).await;
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn tokio_streams() {
    let mut stream =
        tokio_stream::iter(["Let's", "Rock", "n", "Roll"]) // streams are like iterators
            .map(|s| s.to_ascii_uppercase());

    // but they do not implement any iterator trait so for cannot be used
    while let Some(s) = stream.next().await {
        println!("{s}");
    }

    // main use case of Tokio streams is a TcpStream server
}

fn main() {
    serial_tokio();
    println!("----------------------------------");
    tokio_tasks();
    println!("----------------------------------");
    tokio_expensive_computation();
    println!("----------------------------------");
    tokio_streams();
}
