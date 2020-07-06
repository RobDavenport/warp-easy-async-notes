use warp::Filter;

// Extract Params + Combine Routes
pub async fn run() {
    // From Step1, with parameters
    let hello = warp::any()
        .and(warp::path("greeting"))
        .and(warp::path::param::<String>()) // Extract a String and pass it to next line
        .map(|name| format!("Hello, {}!", &name));

    // Use /math/ multiple times
    let math = warp::any().and(warp::path("math"));

    let add = math
        .and(warp::path::param::<i32>()) // Extract an i32, pass it along
        .and(warp::path("plus")) // (i32) still here
        .and(warp::path::param::<i32>()) // Extract another (i32), so we have (i32, i32) now
        .map(|left: i32, right: i32| (left + right).to_string());

    let subtract = math
        .and(warp::path::param::<i32>())
        .and(warp::path("minus"))
        .and(warp::path::param::<i32>())
        .map(subtract); // We can use a function like this!

    let routes = hello.or(add).or(subtract); // We combine routes like this using .or
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn subtract(left: i32, right: i32) -> String {
    (left - right).to_string()
}
