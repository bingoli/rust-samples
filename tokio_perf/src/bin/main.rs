extern crate tokio;
extern crate chrono;
#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use tokio::{prelude::*, runtime, timer::Interval};
use chrono::prelude::*;

lazy_static! {
    static ref MY_RUNTIME: runtime::Runtime = runtime::Builder::new()
        .core_threads(1)
        .blocking_threads(10)
        .keep_alive(Some(Duration::from_secs(60)))
        .name_prefix("tokio-thread")
        .after_start(|| {
        })
        .before_stop(|| {
        })
        .build()
        .expect("Initialize runtime failed");

    static ref TIME_COUNT: RwLock<i32> = {
        RwLock::new(0)
    };
}


fn test_interval() {
    let interval = Duration::from_millis(1000);
    let task = Interval::new_interval(interval)
        .for_each(move |_| {

            let count = {
                let mut w = TIME_COUNT.write().unwrap();
                *w += 1;
                w
            };
            let my_time = Utc::now();
            println!("{}: {}", count, my_time.format("%T"));
            Ok(())
        })
        .then(|ret| {
            if let Err(e) = ret {
                println!("run task failed: {:?}", e);
            }
            Ok(())
        });
    
        MY_RUNTIME.executor().spawn(task);
}

fn main() {
    test_interval();
    
    loop {
        let sleep_duration = Duration::from_secs(60*60*1000);
        thread::sleep(sleep_duration);
    };
}