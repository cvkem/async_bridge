use async_bridge::run_async;



fn main() {
    println!("\nSynchronous Main started now running some Async code");
    async_bridge::run_async(async {
        println!("Running an async function from within synchronous code");
    });
    
    async_bridge::run_async(async {
        println!("Running an async function from within synchronous code (second run)");
    })

}