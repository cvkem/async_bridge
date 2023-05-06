use std::thread;
use async_bridge;


/// the type signature of the Closure we want to pass through layers of Synchronous code.
// The closure needs to be boxed in order to turn it into a Sized type that can be passed around on the stack
type ClosureType = Box<dyn Fn(i32) -> i32>;

#[tokio::main]
async fn main() {
    // show the actions of the async-bridge (for analysis only).
    async_bridge::set_console_logging(true);

    println!("Build the closure to call from within a synchronous library/layer.");

    let closure_to_call = construct_async_closure(2);

    println!("\nNow passing the closure to the synchronous library to be executed later (one or more times)");
    synchronous_library_layer(closure_to_call);
    println!("READY.")
}



/// construct the closure based on the input 'multiplier' which executes code in an asynchronous context.
fn construct_async_closure(multiplier: i32) -> ClosureType {
    // Create the closure
    let closure = move |input| {
        let action = async move{
            println!("Now running in an asynchronous context");
            // performing the (not really) asynchronous tasks based on input parameters from:
            //  * multiplier is obtained from the creator of the Closure.
            //  * input passed from within the asynchronous code
            input * multiplier
        };
        // execute the customer action created above
        async_bridge::execute_action(action)
    };
    // The closure needs to be boxed in order to turn it into a Sized type that can be passed around on the stack
    Box::new(closure)
}

fn synchronous_library_layer(closure_to_call: ClosureType) {
    println!("\nAsynchronous Main started now in the middle of a synchronous library.");

    println!("Calling some async code via the Closure");
    let input = 21;
    let result = closure_to_call(input);
    println!("The asynchrous closure called with input {input} returned result {result}.")
}
