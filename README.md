# async_bridge
Asynchronous processing has [significant advantages](./when_to_program_asynchronous.md), but also brings [some challenges](./pitfalls_of_async.md). In practise we often need to combine asynchronous and synchronous code. The async_bridge helps you in combining these two programming models more easily.


## Challenges of mixing Syncronous and asynchronous code
Often the initial answer to the question on how to mix synchronous and asynchoronous code is: _"DO NOT mix. Just rewrite your solution to be fully asynchronous"_. This might be the right option if you have battle-tested async libraries that cover all your needs. However, this does not cover all use-cases. 

In case that you are forced to mix synchronous and asynchronous code, the tokio documentation suggests the next solution-patterns: [bridging to async code](https://tokio.rs/tokio/topics/bridging):

1. Build a fully asynchronous library and add a synchronous interface (preferred solution).
2. Creating a Tokio-runtime in your synchronous program (or libary) and either:
    * Block_on this runtime such that you synchronous waits for the asynchronous task to finish
    * Spawn your asynchronous tasks on this runtime, such that your synchronous program continues. You get back a join-handle that you can use to check whether the asynchronous task has finished and to retrieve te result.
3. Start a Tokio-runtime and move it to a separate thread. Next you use a channel to submit blocking tasks to the runtime. You need to define a message format and a set up a custom handler to trigger the corresponding asynchronous task.

The pattern 1 focussed on library developers that primarily work with asychronous code and add a synchronous interface to your library. So this pattern is about making async available in a synchronous context.
 
The patterns 2 is focussed on calling asynchronous code (or libraries) in a synchronous context in a blocking or a non blokinhg manner.

Pattern 3 can be used by both library developers to deliver a message interface to an asynchronous library or by library consumers to build a clean and decoupled intrface to asynchronous code.

For the implementation details see [bridging to async code](https://tokio.rs/tokio/topics/bridging).


### Limitations of the three standard approaches to mixing Sync and Async
All the standard approaches, mentioned in the previous section, have their use-cases and limitations:

1. Building a synchronous interface can defeat the advantages of asynchronous programming:
    * unless a single call results in many asynchronous tasks such that the runtime has a backlog of activities to process. 
    * in case you use multiple asynchronous libraries via their synchronous interface, each of these libraries will likely spin-up its own runtime.

2. When using the blocking interface for small tasks you end up having to little other tasks to fill up the gaps. When opting for the Spawning of taks (option 2b) you will have to juggle with JoinHandles to get the responses.

3. This option does not handle return errors, and does not pass back errors that occur after the Task has been received at the other end of the channel.

Next we will describe the async_bridge crate with provides an alternative solution that resolves part of these limitations.


## The Async_bridge crate
The async_bridge is a small crate build on top of Tokio, the most used async-runtime in Rust. It allows you to mixing synchronous and asynchronous code can easily and reduces the chances that you shoot yourself in the foot.

### How sync_bridge operates

The async bridge is used to run asynchronous tasks in a synchronous context efficiently. In order to do so it tries to find a handle to the existing Tokio runtime (of the current tread). If this runtime is discovered the asynchronous action (task) is ran on this runtime. If no runtime is found a multithreaded runtime is created (named "Async-bridge runtime") and stored in a local static. So the async_bridge will only spin up one Tokio runtime and it can be used with asynchronous programs that switched to a synchronous context (used a synchronous library)

### Async_bridge interface
The library consists of three functions that all should be run in a synchronous context (process):
* run_async: run a future (asynchronous task) to completion and return the result of that task.
* spawn_async: run a future on a diffent thread and return a JoinHandle.
* handle_await: Used to await a JoinHandle as returned by 'spawn_async'.
* execute_action: Can be used to build a closure that captures the async runtime such that it can execute asynchronous code in a synchronous context.

### Usecases for the async_bridge
The use-cases of async_bridge are (in my opinion):

* When you need an asynchronous program but also need pieces of syncrhonous code, but later step back to asynchronous again (for example to handle IO-operations asynchronous)
* When building a library that needs async but you do not know whether this library is used with an synchronous or a synchronous main program.
* When you have a mainly sychronous progam, but you want to spawn some asynchronous task to use the benefits of async program for some IO-sensitive parts (No need to manage the initialization of the async-runtime and passing around handles. aync_bridge::run_async will do the works for you)
* Making the advantages of async-IO easily available in a synchronous context (provide your problem benefits from multiple IO-operations being executed in parallel).


The advantages of the async_bridge over the standard patterns:

* The sync_bridge is usable with both synchronous and in asynchronous main programs:
    * The async_bridge reuses the existing Tokio-runtime in case of an asynchronous program (when available).
    * When no Tokio-runtime is found it will start a new multi-threaded runtime exactly once.
* No need to manage a runtime handle that needs to be passed around in your program.
* The sync_bridge takes care that the runtime is created once, and can be shared across code and libraries that use the async_bridge.


### The included examples
In the examples folder of this project I have included three small examples of using the async_bridge:
1. **main_async**: Shows how you can jump back to asynchronous mode (execute asynchronous tasks) when you are one of more layers deep in synchronous code. The async runtime of your main is used to execute the task (unless you are on a spawned thread.)  
2. **main_sync**: Shows how async_bridge::run_async operates well in an synchronous context, where async_bridge starts the Tokio async runtime exactly once and only when needed
3. **spawn**:An example of a program that spawns asynchronous tasks in a synchronous program and later checks if the results are available. This can be used for example to interact with AWS-S3 as the primary library for that is asynchronous (which is the natural fit for this usecase).
4. **pass_closure**: shows how a clojure containing asynchronous can be build that will be executed later in a synchronous context. The closure can take parameters from its creator and also get parameters when called. 

The examples 1 and 2 also show that the async runtime is not visible when you run code on a thread that has been spawned. So the async runtime is only visible on the thread were it was started, and on the threads that of thee threadpool of that runtime (in case of a multi-threaded runtime).



