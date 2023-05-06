use std::thread;
use async_bridge;



fn main() {
    // show the actions of the async-bridge (for analysis only).
    async_bridge::set_console_logging(true);

    perform_two_async_calls("MAIN");

    println!("NOTE-1: The async_bridge did start a runtime, as there is no Main-runtime of the program.");
    println!("NOTE-2: The second call also runs via the Fall-back path (so Handle::try_current() does not detect the fall-back runtime).");

    println!("\nNow runnning the same code on another thread:");
    thread::Builder::new()
        .name("Spawned".to_owned())
        .spawn(|| { perform_two_async_calls("SPAWNED thread");})
        .expect("Failed to start the Thread")
        .join()
        .expect("Failed to join");

        println!("NOTE-3: The async_bridge reuses the same Runtime even when called from another thread (as expected).");
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
