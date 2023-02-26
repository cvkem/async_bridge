use async_bridge::run_async;


#[tokio::main]
async fn main() {
    println!("\nAsynchronous Main started now running some Async code");
    async_bridge::run_async(async {
        println!("Running an async function from within synchronous code");
    });

    async_bridge::run_async(async {
        println!("Running an async function from within synchronous code (second run)");
    })
}
