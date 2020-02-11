#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::time:: {
    Instant
};

#[macro_export]
macro_rules! record_time_cost {
    ($label:expr) => {
        let _time_recorder = $crate::TimeRecorder::new($label);
    };
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct TimeRecorder {
    start_time: Instant,
    label: String
}

impl TimeRecorder {
    pub fn new(label: &str) -> Self {
        TimeRecorder {
            start_time: Instant::now(),
            label: label.to_owned()
        }
    }
}

impl Drop for TimeRecorder {
    fn drop(&mut self) {
        let duration_ms = self.start_time.elapsed().as_millis();
        println!("{} cost time: {} ms", self.label, duration_ms);
    }
}