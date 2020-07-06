use warp::Filter;

// Hello World!
pub async fn run() {
    // Build our first route!
    let hello = warp::any().map(|| "Hello, world!".to_string());

    // Run the server!
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await; //Why do we need to await here?
}
