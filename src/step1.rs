use warp::Filter;

// Hello World!
pub async fn run() {
    let hello = warp::any().map(|| "Hello, world!".to_string());

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
