# Pitfalls of asynchonous programming

I have been working on a project involving handling Parquet-files on S3. S3-operations are a good candidate for asynchronous programming, and Parquet is about big-data processing, so that is the type of stuff you like to store in a low cost storage like S3. For this project I used [AWS-rust-S3 library](https://crates.io/crates/aws-sdk-s3) (fully asynchronous) and the [reference implementation of Parquet](https://github.com/apache/arrow-rs/blob/master/parquet/README.md) which only has limited async support.


## Single-threaded code can halt easily
Soon I ran into issues as my tests seemed to fail. However, when calling the same code from a main program to inspect my code with a debugger everything seemed to be fine. This is one of the programmers nightmares as it means that the bug vanishes when you try to observe it, and re-appears when you look the other way. After studying some documentation I noticed that tokio tests are [ran on a single-threaded async engine when using the tag ](https://docs.rs/tokio/latest/tokio/attr.test.html) when using the default atrribute:

`[tokio::test]` 

while the attribute that you apply to [your main functions](https://docs.rs/tokio/latest/tokio/attr.main.html) is multi-threaded by default:

`#[tokio::main]`

In the end the solution was fairly simple. After changing my tests to use a multi-threaded async runtime via the attribute:

`#[tokio::test(flavor = "multi_thread")]`

my tests did run without errors.

This does not explain yet why the code failed on a single-threaded async runtime. However, my assumption is that one of the libraries I used did execute some blocking code. As a result the program might halt because:
* it is waiting forever on another part of the program that can not continue.
* the async runtime might miss some timers or OS-events due to the fact that the thread that handles these events is blocked for too long.

## Jumping from synchronous code back to an asynchronous context
Within my asynchronous program I also included some synchronous code. As this code did not take too much time, this should not cause any issues. However, I also needed to do some file operations in my synchronous, which are slow. [Slow (or blocking) sub-processes](https://ryhl.io/blog/async-what-is-blocking/) can wreak havoc on a asynchronous program, so I tried to use async file-operations, as the logical option would be to run these file [operations asynchronous](https://docs.rs/tokio/latest/tokio/fs/) by jumping from my synchronous code back to asynchronouse code. This helps as my synchronous code becomes async again and does a clean await. 
**Right?**...
Nope, this is wrong for two reasons:

1. in case the synchronous code was part of an async-thread this thread still gets blocked. So even though jump back to asynchronous code, this code will be running on one of the other threads of asynchronous thread pool (which lost one thread). 
2. In order to execute an asynchronous task you need to have a handle to the runtime. So you either have to carry the handle from the original asynchronous code through the synchronous code to be able to jump back to asynchronous code, or you need to perform some tricks to recover the handle or start a new runtime [(see async_bride)](./README.md).

### When does jumping back to asynchronous make sense
Does the previous section mean that we should never jump back to asynchronous code? **Well** .... that depends. I see two good reasons to jump back from synchronous code to asynchronous:

1. Jumping back to asynchronous code to read a single file does not make much sense, as mentioned above. However, if you have a large tasks that splits out in multiple asynchronous task that can be performed simultaneously it is useful to jump back to asynchronous code. So if you have a sizable task to perform asynchronous it does make sense.
2. In some case you only have an asynchronous library. For example when you need to access S3 the default option is a asynchronous library (which is logical as S3 access is a spot-on use-case for asynchronous processing, especially if you code needs to connect to S3 over the (relatively slow) internet.).


## `Futures::executor::block_on` is no good solution
Initially I had the incorrect assumption (read misconception) that [`futures::executor::block_on`](https://docs.rs/futures/latest/futures/executor/fn.block_on.html) was my handle to the original async runtime and thus would give me the option to jump back to the original async world after deviating into some synchronoous code. However that asumption is incorrect. If you study the [documentation]((https://docs.rs/futures/latest/futures/executor/fn.block_on.html)) you notice that `futures::executor::block_on` runs the future on the current thread. And as we saw earlier on this page, single-threaded runtime could cause surprises.

You can switch `futures::executor::block_on` to use a [local-thread-pool](https://docs.rs/futures/latest/futures/executor/struct.LocalPool.html)). However, in that case you a configuring a second async runtime next to the Tokio async runtime, which sounds like a bad plan.

## Debugging asynchronous code can be hard
The project involving processing Parquet files on S3 involved two sizeable external dependencies (on for [S3](https://crates.io/crates/aws-sdk-s3) and one for [Parquet](https://github.com/apache/arrow-rs/blob/master/parquet/README.md)). The Parquet library also had significant synchronous part, so I had enough ingredients for a rough ride. When my program halted the experts adviced me to use the [tokio-console, a debugger for async code,](https://github.com/tokio-rs/console) to debug my program. I agree this is a good tool for get a lot of information out of you program provided the next conditions are met:

1. The error shows up in the part of the code-base you understand.
2. You code is fully asynchronous.

Neither of these conditions applied in my case. I was using two complex libraries and the Parquet library was mostly symchronous. 

I was not able to debug it, and in the end I developed the [async_bridge](./README.md) to have a structured was of combining synchronous and asynchonous code, which solved my isssue.

