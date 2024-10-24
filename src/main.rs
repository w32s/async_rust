mod shared;
mod fileread;
mod http;

use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //sync_http_calls().await?;
    //async_http().await?;
    //single_threaded_fib().await?;
    //multi_threaded_fib().await?;
    //shared::sharing_data_between_threads().await;
    //fileread::file_demo().await;
    //http::async_http_example().await?;
    //shared::poisoned_mutex().await;
    //http::async_http_example().await?;

    Ok(())
}
// Action 1
async fn sync_http_calls() -> Result<Result<(), Error>, Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();

    let first = reqwest::get(url);
    let second = reqwest::get(url);
    let third = reqwest::get(url);
    let fourth = reqwest::get(url);

    let _first = first.await?;
    let _second = second.await?;
    let _third = third.await?;
    let _fourth = fourth.await?;
    let elapsed_time = start_time.elapsed();

    println!("Requests took {} ms", elapsed_time.as_millis());

    Ok(Ok(()))
}
// Action 2
async fn async_http() -> Result<Result<(), Error>, Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();

    let (_, _, _, _) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url)
    );
    let elapsed_time = start_time.elapsed();

    println!("Requests took {} ms", elapsed_time.as_millis());

    Ok(Ok(()))
}
// Action 3
async fn single_threaded_fib() -> Result<(), Error> {
    let start_time = Instant::now();

    let _ =  fibonacci(40);

    let elapsed_time = start_time.elapsed();

    println!("Sequence took {} ms", elapsed_time.as_millis());

    Ok(())
}

// Action 4
async fn multi_threaded_fib() -> Result<(), Error> {
    let start_time = Instant::now();

    let mut handles : Vec<JoinHandle<u64>> = vec![];
    for _ in 0..4{
        let handle = thread::spawn(|| {
           return fibonacci(40)
        });
        handles.push(handle);
    }
    for h in handles{
       let res = h.join().unwrap();
        println!("Thread finished with result: {}", res);
    }
    let elapsed_time = start_time.elapsed();

    println!("Sequence took {} ms", elapsed_time.as_millis());

    Ok(())
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n-1) + fibonacci(n-2)
}