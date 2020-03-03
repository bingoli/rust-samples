extern crate sqlite_perf;

use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn hello_devworld() {
    println!("Hello, World!");
}

#[no_mangle]
pub extern "C" fn run_sqlite_perf_test(database_url: *const c_char) {
    use sqlite_perf::run_test;
    let database_url = unsafe { CStr::from_ptr(database_url) };
    let database_url = database_url.to_str().unwrap();
    run_test(database_url);
}

#[no_mangle]
pub extern "C" fn run_file_perf_test(database_url: *const c_char) {
    use file_perf::run_test;
}