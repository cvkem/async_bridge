use std::thread;
use async_bridge;



fn main() {
    // show the actions of the async-bridge (for analysis only).
    async_bridge::set_console_logging(true);

    perform_two_async_calls("MAIN");

    println!("Please note that async_bridge did start a runtime.");

    println!("\nNow runnnin the same code on another thread:");
    thread::Builder::new()
        .name("Spawned".to_owned())
        .spawn(|| { perform_two_async_calls("SPAWNED thread");})
        .expect("Failed to start the Thread")
        .join()
        .expect("Failed to join");

        println!("Please note that async_bridge already started a runtime for the main thread so no new runtime started here.");
}


fn perform_two_async_calls(prefix: &str) {
    println!("\nSynchronous Main started now running some Async code");
    async_bridge::run_async(async {
        println!("{prefix}: Running an async function from within synchronous code");
    });
    
    async_bridge::run_async(async {
        println!("{prefix}: Running an async function from within synchronous code (second run)");
    });
}
