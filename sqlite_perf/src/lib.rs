#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod perf_test;

use crate::perf_test::run_cases;

pub fn run_test() {
    run_cases();
}