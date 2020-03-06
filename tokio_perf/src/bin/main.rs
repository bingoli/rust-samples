
extern crate tokio_perf;
use std::thread;

use std::time::Duration;

use tokio_perf::perf_test::test_interval;

fn main() {
    test_interval();

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}