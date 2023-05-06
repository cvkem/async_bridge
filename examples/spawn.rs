use async_bridge;
use tokio::time::{sleep, Duration};


fn main() {
    println!("\nSynchronous Main started now running some Async code via spawn thread.");

    with_custom_await();

    println!("\nNow doing the same but slightly more concise with the async_bridge::handle_await");

    async_await();
}


async fn return_meaning() -> i32 {
    println!("Running an async function from within synchronous code");
    sleep(Duration::from_millis(1000)).await;
    println!("Running an async function from within synchronous code (waited 1 second)");
    42
}


/// example of building an async function used to await the spawned task.
fn with_custom_await() {
    let join_handle = async_bridge::spawn_async(return_meaning());

    // custom await function
    println!("NOTE-1: In custom code you roll your own async function that handles the output on the remote thread before returning");   
    async_bridge::run_async(async {
        println!("Now await the handle...");
        match join_handle.await {
            Ok(result) => println!(" The returned meaning {result}"),
            Err(err) =>  println!("Failed with error {err:?}")
        }
    })

}

/// 
fn async_await() {
    let join_handle = async_bridge::spawn_async(return_meaning());

    println!("The task has been spawned");

    println!("{}\n\t{}\n\t{}\n\t{}",
        "NOTE-2: when using async_bridge:",
        "* You retrieve the result directly from the join-handle (no async function needed)",
        "* and you process the result on the current thread.",
        "* in case of a failure this was already reported by async_bridge::handle_await!");
    match async_bridge::handle_await(join_handle) {
        Ok(result) =>  println!("The result returned by the join-handle is {result}"),
        Err(_err) => println!("Joining failed and the error has been reported already on the console!!! So I do not repeat it here.")
    }
}