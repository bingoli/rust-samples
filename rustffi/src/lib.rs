extern crate sqlite_perf;

#[no_mangle]
pub extern "C" fn hello_devworld() {
    println!("Hello, World!");
}

#[no_mangle]
pub extern "C" fn run_sqlite_perf_test() {
    use sqlite_perf::run_test;
    run_test();
}