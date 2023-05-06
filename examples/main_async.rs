use std::thread;
use async_bridge;


#[tokio::main]
async fn main() {
    // show the actions of the async-bridge (for analysis only).
    async_bridge::set_console_logging(true);

    perform_two_async_calls("MAIN");

    println!("\nNow runnning the same code on another thread:");
    thread::Builder::new()
        .name("Spawned".to_owned())
        .spawn(|| { perform_two_async_calls("SPAWNED thread");})
        .expect("Failed to start the Thread")
        .join()
        .expect("Failed to join");
    println!("NOTE-1: The async_bridge could not find the main-runtime on this thread, so a second runtime is started.");
    println!("NOTE-2: The second runtime is started once, so the same runtime is used for both invocations.");
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
