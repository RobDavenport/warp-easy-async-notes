use warp::Filter;

// Blocking code!
pub async fn run() {
    // From Step2
    let hello = warp::any()
        .and(warp::path("greeting"))
        .and(warp::path::param::<String>())
        .map(|name| format!("Hello, {}!", &name));

    let factorial = warp::any()
        .and(warp::path("factorial"))
        .and(warp::path::param::<u64>())
        .map(factorial);

    let routes = hello.or(factorial);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// This is not async
fn factorial(number: u64) -> String {
    super::really_slow_code(); // Simulate slow function or blocking I/O
    let result: u64 = (1..=number).product();
    format!("{}", result)
}
