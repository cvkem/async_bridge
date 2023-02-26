
use std::{
    future::Future,
    sync::Mutex};
use tokio;
//extern crate lazy_static;
use lazy_static::lazy_static;


struct RuntimeConfig {
    num_worker_threads: usize,
    num_blocking: usize
}

/// 
const RT_CONFIG: Mutex<RuntimeConfig> = Mutex::new(RuntimeConfig{
            num_worker_threads: 4, 
            num_blocking: 3});

lazy_static! {
    static ref RT: tokio::runtime::Runtime = (|| {
        let thread_name = "Async-bridge runtime"; 
        println!("Running a closure to create a runtime via Lazy_static!");
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
        Ok(_handle) => {
           let handle = _handle.clone();
           tokio::task::block_in_place(move || handle.block_on(action))
        }
        // No async runtime, so create one and launch this task on it 
        Err(_err) => {
            RT.block_on(action)
        }
    }
}

/// Check if an executor is available and run action on this executor, otherwise start a runtime to run and block_on the async action
pub fn spawn_async<F>(action: F) -> tokio::task::JoinHandle<F::Output> 
where F: Future + Send + 'static,
       F::Output: 'static + Send {

    match tokio::runtime::Handle::try_current()  {
        Ok(handle) => {
            let _guard = handle.enter();
            handle.spawn(action)
        }
        // No async runtime, so create one and launch this task on it 
        Err(_err) => {
            RT.spawn(action)
        }
    }
}