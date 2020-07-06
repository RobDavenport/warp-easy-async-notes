use warp::Filter;

// Blocking code - Fixed!
pub async fn run() {
    // From Step2
    let hello = warp::any()
        .and(warp::path("greeting"))
        .and(warp::path::param::<String>())
        .map(|name| format!("Hello, {}!", &name));

    let factorial = warp::any()
        .and(warp::path("factorial"))
        .and(warp::path::param::<u64>())
        .and_then(factorial);

    let routes = hello.or(factorial);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// What is the output?
async fn factorial(number: u64) -> Result<impl warp::Reply, std::convert::Infallible> {
    let task = tokio::task::spawn_blocking(move || {
        super::really_slow_code();
        let result: u64 = (1..=number).product();
        result
    });

    let result = task.await.unwrap();
    Ok(format!("{}", &result))
}