use std::sync::RwLock;
use std::time::{Duration, Instant};

use tokio::{prelude::*, runtime, timer::Interval};
use chrono::prelude::*;

lazy_static! {
    static ref MY_RUNTIME: runtime::Runtime = runtime::Builder::new()
        .core_threads(1)
        .name_prefix("tokio-thread")
        .build()
        .expect("Initialize runtime failed");

    static ref TIME_COUNT: RwLock<i32> = {
        RwLock::new(0)
    };
}

pub fn test_interval() {
    let interval = Duration::from_millis(1000);
    let task = Interval::new_interval(interval)
        .for_each(move |deadline| {
            let count = {
                let mut count = TIME_COUNT.write().unwrap();
                *count += 1;
                count
            };
            println!("{}, {}, now: {:?}, deadline: {:?}", count, Utc::now().format("%T"), Instant::now(), deadline);
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