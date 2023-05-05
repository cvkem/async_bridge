use std::thread;
use async_bridge;


#[tokio::main]
async fn main() {
    // show the actions of the async-bridge (for analysis only).
    async_bridge::set_console_logging(true);

    perform_two_async_calls("MAIN");

    println!("\nNow runnnin the same code on another thread:");
    thread::Builder::new()
        .name("Spawned".to_owned())
        .spawn(|| { perform_two_async_calls("SPAWNED thread");})
        .expect("Failed to start the Thread")
        .join()
        .expect("Failed to join");
    println!("Please note that async_bridge could not find the main-runtime ")
}



fn perform_two_async_calls(prefix: &str) {
    println!("\nAsynchronous Main started now running some Async code");
    async_bridge::run_async(async {
        println!("{prefix}: Running an async function from within synchronous code");
    });

    async_bridge::run_async(async {
        println!("{prefix}: Running an async function from within synchronous code (second run)");
    });
}
