use warp::Filter;

// Extract Params + Combine Routes
pub async fn run() {
    // From Step1, with parameters
    let hello = warp::any()
        .and(warp::path("greeting"))
        .and(warp::path::param::<String>())
        .map(|name| format!("Hello, {}!", &name));

    // Use /math/ multiple times
    let math = warp::any().and(warp::path("math"));

    let add = math
        .and(warp::path::param::<i32>())
        .and(warp::path("plus"))
        .and(warp::path::param::<i32>())
        .map(|left, right| {
            let result = left + right;
            format!("{}", result)
        });

    let subtract = math
        .and(warp::path::param::<i32>())
        .and(warp::path("minus"))
        .and(warp::path::param::<i32>())
        .map(subtract);

    let routes = hello.or(add).or(subtract);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn subtract(left: i32, right: i32) -> String {
    (left - right).to_string()
}
