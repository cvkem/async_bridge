# When to use asynchronous programming
Nowadays we want to make everything more 'green'. The same holds for computing which consumes quite a bit of energy. [Rust is one of the most energy efficient languages](https://www.devsustainability.com/p/paper-notes-energy-efficiency-across-programming-languages#:~:text=The%20results%20show%20that%20C,most%20energy%20efficient%20programming%20language.). Especially when combined with asynchronous programming, as the asynchronous model allows for more efficient web-server processes.

## The usecase of asynchronous programming
The async programming model does offer significant advantages in case of IO-bound computation. Your processor is much faster than the fastest SSD storage,
and reaching out to storage over the internet, such as AWS-S3 or GCP Cloudstorage is even slower. For the processor of your computer waiting for IO seems takes aeons. Of course you can off-load the IO to a separate OS-thread, such that the main thread continues doing work. This unblocks the processor to do other activities but has two limitations:

1. Does you program has enough other tasks to handle while waiting for events to happen (for example a harddisk operation, an https-response or even worse a user providing key-board input.).
2. The context-switching between OS-threads does not come for free, and introduces a significant overhead.

This is where Asynchronous programming has its sweet spot as it solves both of these problems. In an async program all computation consists of futures (future computations) which will be processed when needed in Rust (as Rust has a lazy computational model, where futures are only evaluated when someone requests a results. This contrasts with most other languages such as Node or Java, where futures start running/processing immediately such that you get computation performed in parallel). As the concept of future is deeply entrenched in async programming a future often consist of many futures that contain other futures, usually resulting in a large number of tasks (futures) that can be processed (solving issue 1). 
The async programming model introduces, so called, green threads which are managed by the process itself. This kind of management is more efficient than OS-threads as they do not require a jump to the OS-kernel and a more efficient way to store and restore processor state during switches (The program (or the compiler) can use optimizations that are not available to the OS-kernel), so this solves issue  

## The limitations of asynchronous programming

Although this all sounds like significant advantages there are also a few downsides of Asynchronous programming, i.e.:

* Asynchronous programming implies distributed computation, which is inherently more complex as you need to think through that computation can happen in parallel, and you need safeguards like mutexes and atomic operations to ensure that your program works correctly.
* Asynchronous model is based on collaborative multi-tasking, which implies that all processes should yield control on a regular basis to the async-runtime. As a rule of thumb process should spend [at most 10-100 microseconds between subsequent '.await' operations](https://ryhl.io/blog/async-what-is-blocking/) so do not run blocking operations on your async threads such as:
    * Compute intensive operations
    * Synchronous-IO 
* Asynchronous programming in Rust had quite [a few pitfalls](./pitfalls_of_async.md) for me, and I guess I am not the only one.
* Many libaries are written for synchronous programming, and these libraries do not blend easily with Asynchronous code.
* Asynchronous Rust code is a bit more verbose as each future also needs to be awaited (immediately or later in time to extract the result.) 

Part of the limitations mentioned here are [resolved by the async_bridge](./README.md).