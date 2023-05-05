
use std::{
    future::Future,
    sync::{atomic::{AtomicBool, Ordering},
    Mutex}, 
    thread::{self, JoinHandle}};
use tokio;
use lazy_static::lazy_static;


static CONSOLE_LOGGING: AtomicBool = AtomicBool::new(false);

pub fn set_console_logging(value: bool) {
    CONSOLE_LOGGING.store(value, Ordering::Relaxed);
}

/// report the current thread and the thread used to run async-code for analyis purposes.
/// (should only be called when CONSOLE_LOGGING is set to true)
fn report_threads(handle: tokio::runtime::Handle, operation: &str) {
    let curr_thread_name = format!("{:?}", thread::current().name());
    let action = async { format!("{:?}", thread::current().name())};
    let async_thread_name = tokio::task::block_in_place(move || handle.block_on(action));
    println!("ASYNC_BRIDGE-{operation}: In thread '{curr_thread_name:?} found existing handle on thread: '{async_thread_name:?}'.");
}


struct RuntimeConfig {
    num_worker_threads: usize,
    num_blocking: usize,  // intended to check the number of blocked threads, which should never exceed the number of worker_threads -1.
}

/// Configuration behind a mutex, but not really needed as the code that uses this data is executed only once. However, needed to please the compiler.
const RT_CONFIG: Mutex<RuntimeConfig> = Mutex::new(RuntimeConfig{
            num_worker_threads: 4, 
            num_blocking: 3});

lazy_static! {
    static ref RT: tokio::runtime::Runtime = (|| {
        let thread_name = "Async-bridge runtime";
        if CONSOLE_LOGGING.load(Ordering::Relaxed) {
            println!("ASYNC_BRIDGE: Starting a Runtime on a thread named '{thread_name}' via Lazy_static!");
        } 
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(RT_CONFIG.lock().unwrap().num_worker_threads)
            .thread_name(thread_name)
            .build()
            .unwrap();

        // should be behind a feature flag I guess
        // rt.block_on(async {
        //     use console_subscriber;
        //     println!("Starting the console-subscriber for Tokio-console for thread '{}'", thread_name);
        //     console_subscriber::init();        
        // });
        rt
    })();
}


/// Check if an executor is available and run action on this executor, otherwise start a runtime to run and block_on the async action
pub fn run_async<F>(action: F) -> F::Output 
where F: Future { 

    match tokio::runtime::Handle::try_current()  {
        Ok(handle) => {
            let handle = handle.clone();
            if CONSOLE_LOGGING.load(Ordering::Relaxed) {
                report_threads(handle.clone(), "run_async");
            } 
            tokio::task::block_in_place(move || handle.block_on(action))
        }
        // No async runtime, so create one and launch this task on it 
        Err(_err) => {
            if CONSOLE_LOGGING.load(Ordering::Relaxed) {
                report_threads(RT.handle().clone(), "run_async");
            } 
            RT.block_on(action)
        }
    }
}

/// Check if an executor is available and run action on this executor, otherwise start a runtime to run and block_on the async action.
/// A spawned process starts execution immediately on a separate (green) thread. 
/// and a JoinHandle it returned to the calling thread such that this thread can continue execution.
/// When the result of the spawned process is needed the 'handle_await' can be applied to retrieve either
/// a result or an error from the JoinHandle.
pub fn spawn_async<F>(action: F) -> tokio::task::JoinHandle<F::Output> 
where F: Future + Send + 'static,
       F::Output: 'static + Send {

    match tokio::runtime::Handle::try_current()  {
        Ok(handle) => {
            let _guard = handle.enter();
            if CONSOLE_LOGGING.load(Ordering::Relaxed) {
                report_threads(handle.clone(), "spawn_async");
            } 
            handle.spawn(action)
        }
        // No async runtime, so create one and launch this task on it 
        Err(_err) => {
            if CONSOLE_LOGGING.load(Ordering::Relaxed) {
                report_threads(RT.handle().clone(), "spawn_async");
            } 
            RT.spawn(action)
        }
    }
}

/// Used to await a tokio JoinHandle in a synchroneous context. In case of an error, the error is printed to the console.
pub fn handle_await<F>(join_handle: tokio::task::JoinHandle<F>) -> Result<F, tokio::task::JoinError> {
    if CONSOLE_LOGGING.load(Ordering::Relaxed) {
        let handle = match tokio::runtime::Handle::try_current()  {
            Ok(handle) => handle.clone(),
            Err(_err) => RT.handle().clone()
        };
        report_threads(handle, "handle_await");
    } 
    run_async(async {
        match join_handle.await {
            Ok(result) => Ok(result),
            Err(err) =>  {
                println!("Handle-await: Failed with error {err:?}");
                Err(err)
            }
        }    
    })
}