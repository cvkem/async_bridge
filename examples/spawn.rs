use async_bridge::run_async;
use tokio::time::{sleep, Duration};

async fn return_meaning() -> i32 {
    println!("Running an async function from within synchronous code");
    sleep(Duration::from_millis(1000)).await;
    println!("Running an async function from within synchronous code (waited 1 second)");
    42
}


fn with_custom_await() {
    let join_handle = async_bridge::spawn_async(return_meaning());

    // custom await function   
    async_bridge::run_async(async {
        println!("Now await the handle...");
        match join_handle.await {
            Ok(result) => println!(" The returned meaning {result}"),
            Err(err) =>  println!("Failed with error {err:?}")
        }
    })

}

fn async_await() {
    let join_handle = async_bridge::spawn_async(return_meaning());

    println!("The taks has been spawned");

    match async_bridge::handle_await(join_handle) {
        Ok(result) =>  println!("The result returned by the join-handle is {result}"),
        Err(err) => println!("Joining failed with error {err:?}")
    }
}

fn main() {
    println!("\nSynchronous Main started now running some Async code");

    with_custom_await();

    println!("\nNow doing the same but slightly more concise with the async_bridge::handle_await");

    async_await();
}