extern crate rustffi;

use rustffi::*;

fn main() {
    /*
    hello_devworld();

    use std::ffi::CString;

    let database_url = CString::new("demo.db").expect("CString::new failed");
    let ptr = database_url.as_ptr();
    run_sqlite_perf_test(ptr);    
    */
    run_tokio_perf_test();
}