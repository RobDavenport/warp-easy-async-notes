pub mod step1;
pub mod step2;
pub mod step3;
pub mod step4;
pub mod step5;
pub mod step6;
pub mod step7;

#[tokio::main]
async fn main() {
    step1::run().await; // Hello world!
    // step2::run().await; // Add more routes, and take parameters!
    // step3::run().await; // Async routes!
    // step4::run().await; // Async and params combined!
    // step5::run().await; // Working with synchronous code
    // step6::run().await; // Async to the rescue?
    // step7::run().await; // Everything together
}

// This code takes a really long time to run
// We will use it to simulate network traffic or heavy computation
// It doesn't use async, so it is...
// BLOCKING or SYNCHRONOUS
pub fn really_slow_code() {
    println!("Loading...");
    for (percent, _outer) in (0..100).enumerate() {
        for _inner in 0..10_000_000 {}
        println!("{}%", percent);
    }
    println!("Done!");
}
