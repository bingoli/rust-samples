extern crate tokio_perf;
use std::thread;
use std::time::Duration;

use tokio_perf::perf_test::test_interval;


fn main() {
    test_interval();
    
    loop {
        let sleep_duration = Duration::from_secs(60*60*1000);
        thread::sleep(sleep_duration);
    };
}