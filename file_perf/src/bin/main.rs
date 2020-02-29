use std::io;
use std::io::prelude::*;
use std::fs::File;

use std::time:: {
    Instant
};

#[macro_export]
macro_rules! record_time_cost {
    ($label:expr) => {
        let _time_recorder = TimeRecorder::new($label);
    };
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

const BIT_COUNT: u64 = 10 * 1024;

pub struct ReadFileCase {
    file: File,
}

impl ReadFileCase {
    fn new() -> Self {
        Self {
            file: File::open("data.txt").unwrap()
        }
    }

fn test_read_file(&mut self) -> io::Result<()> {
    let mut buffer = [0; BIT_COUNT as usize ];
    let mut handle = (&self.file).take(BIT_COUNT);
    handle.read(&mut buffer)?;

    Ok(())
}

fn test_read_file_try_clone(&self) -> io::Result<()> {
    let mut buffer = [0; BIT_COUNT as usize ];
    let file = self.file.try_clone()?;
    let mut handle = file.take(BIT_COUNT);
    handle.read(&mut buffer)?;

    Ok(())
}
}

fn main() -> io::Result<()> {

    let count: i32 = 10000;

    {
        let mut case = ReadFileCase::new();
        record_time_cost!("test_read_file");
        for _ in 0 .. count {
            case.test_read_file()?
        }
    }

    {
        let case = ReadFileCase::new();
        record_time_cost!("test_read_file_try_clone");
        for _ in 0 .. count {
            case.test_read_file_try_clone()?
        }
    }



    Ok(())
}