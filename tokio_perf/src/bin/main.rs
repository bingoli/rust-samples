extern crate chrono;
extern crate tokio;

use chrono::prelude::Utc;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_millis(1000));
    let mut count: i32 = 0;
    loop {
        count += 1;
        let time = interval.tick().await;
        println!("{}, {}, {:?}", count, Utc::now().format("%T"), time);
    };
}