use std::time::Duration;
use warp::Filter;

// Add async calls
pub async fn run() {
    // From Step2
    let hello = warp::any()
        .and(warp::path("greeting"))
        .and(warp::path::param::<String>()) // Extract a String and pass it to next line
        .map(|name| format!("Hello, {}!", &name));

    // For async function, we must use and_then, not map
    let slow_greeting = warp::any().and(warp::path("slow")).and_then(slow);

    let routes = hello.or(slow_greeting);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Must use a func, because async closure is not allowed
// It's possible for an async function to faill, so we must put it inside of Result
async fn slow() -> Result<impl warp::Reply, std::convert::Infallible> {
    tokio::time::delay_for(Duration::from_secs(3)).await;
    Ok("Done waiting!".to_string()) // The function didn't fail, so wrap it in Ok()
}
