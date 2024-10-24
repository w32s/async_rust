use std::time::Duration;
use reqwest::{Error};
use serde::Deserialize;
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
struct Response {
    url: String,
    args: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct PostResponse {
    #[serde(rename = "userId")]
    user_id: u16,
    id: u16,
    title: String,
    body: String,
}
async fn fetch_data(seconds: u64) -> Result<Response, Error> {
    let request_url = format!("https://httpbin.org/delay/{}", seconds);
    let response = reqwest::get(&request_url).await?;
    let delayed_response: Response = response.json().await?;
    Ok(delayed_response)
}

async fn fetch_post() { // Simulate database call
    let request_url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(request_url).await.unwrap();
    let post : PostResponse = response.json().await.unwrap();
    println!("Fetched post: {:?}", post);
}



async fn calculate_last_login() { // Simulate database call
    sleep(Duration::from_secs(3)).await;
    println!("Logged in 2 days ago");
}

// Action 7 (Finale)
pub(crate) async fn async_http_example()-> Result<(), Error> {
    let start_time = std::time::Instant::now();
    let data = fetch_data(5);
    let post = fetch_post();
    let time_since = calculate_last_login();
    let (posts, _, _) = tokio::join!(
        data, time_since, post
    );
    let duration = start_time.elapsed();
    println!("Fetched {:?}", posts);
    println!("Time taken: {:?}", duration);
    Ok(())
}