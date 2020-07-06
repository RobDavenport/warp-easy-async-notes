use std::time::Duration;
use warp::Filter;

// Add async calls with params
pub async fn run() {
    // From Step2
    let hello = warp::any()
        .and(warp::path("greeting"))
        .and(warp::path::param::<String>())
        .map(|name| format!("Hello, {}!", &name));

    let slow_greeting = warp::any().and(warp::path("slow")).and_then(slow);

    let slow_count = warp::any()
        .and(warp::path("counter"))
        .and(warp::path::param::<u64>())
        .and_then(slow_count);

    let routes = hello.or(slow_greeting).or(slow_count);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Must use a func, because async closure is not allowed
async fn slow() -> Result<impl warp::Reply, std::convert::Infallible> {
    tokio::time::delay_for(Duration::from_secs(3)).await;
    Ok(format!("OK!"))
}

async fn slow_count(secs: u64) -> Result<impl warp::Reply, std::convert::Infallible> {
    tokio::time::delay_for(Duration::from_secs(secs)).await;
    Ok(format!("You waited {} seconds!", &secs))
}
