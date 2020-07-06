pub mod step1;
pub mod step2;
pub mod step3;
pub mod step4;
pub mod step5;
pub mod step6;
pub mod step7;

#[tokio::main]
async fn main() {
    step1::run().await;
    // step2::run().await;
    // step3::run().await;
    // step4::run().await;
    // step5::run().await;
    // step6::run().await;
    // step7::run().await;
}

// This code takes a really long time to run
// It doesn't use async, so it is
// BLOCKING or SYNCHRONOUS
pub fn really_slow_code() {
    println!("Loading...");
    for (percent, _outer) in (0..100).enumerate() {
        for _inner in 0..10_000_000 {}
        println!("{}%", percent);
    }
    println!("Done!");
}
