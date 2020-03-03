use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::sync::RwLock;
use std::thread;

use std::time:: {
    Instant,
    Duration
};

use std::collections::{HashMap};

extern crate tokio;
/*
use tokio::prelude::{AsyncRead, Future};
use tokio::timer::Interval;
*/
use tokio::{prelude::*, runtime, timer::Interval};
use tokio::prelude::Future;

extern crate chrono;

use chrono::prelude::*;

#[macro_use]
extern crate lazy_static;

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
        let duration_ms = self.start_time.elapsed().as_micros();
        println!("{} cost time: {} us", self.label, duration_ms);
    }
}

const BUF_SIZE: u64 = 10 * 1024;

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
    let mut buffer = [0; BUF_SIZE as usize ];
    (&self.file).take(BUF_SIZE).read(&mut buffer)?;

    Ok(())
}

fn test_read_file_try_clone(&self) -> io::Result<()> {
    let mut buffer = [0; BUF_SIZE as usize ];
    let file = self.file.try_clone()?;
    file.take(BUF_SIZE).read(&mut buffer)?;

    Ok(())
}
}


lazy_static! {
    static ref COUNT: i32 = 20;
}

lazy_static! {
    static ref MY_CACHES: RwLock<HashMap<i32, String>> = {
        let h = HashMap::new();
        RwLock::new(h)
    };

    static ref MY_INT: RwLock<i32> = {
        RwLock::new(55)
    };

}

fn test_thread() {
    
    {
        let mut w = MY_CACHES.write().unwrap();
        w.insert(1, "test".to_string());
    }

    {
        let mut w = MY_INT.write().unwrap();
        *w = 10;
    }

    //*COUNT = 30;
    println!("thread 1: {}", *COUNT);
    let handler = thread::Builder::new()
        .name("named thread".into())
        .spawn(|| {
            let handle = thread::current();
            assert_eq!(handle.name(), Some("named thread"));
            
            println!("thread 2: {}", *COUNT);

            {
                let r = MY_INT.read().unwrap();
                println!("my_int: {}", *r);
            }

            {
                let map = MY_CACHES.read().unwrap();
                let v = map.get(&1).unwrap();
                println!("my caches : {} ", v);
            }

        })
        .unwrap();

    handler.join().unwrap();
}

fn test_aync_file() {

    /*
    let task = tokio::fs::File::open("data.txt")
        .and_then(|mut file| {
            let mut contents = vec![];
            file.read_buf(&mut contents)
                .map(|res| {
                    println!("{:?}", res);
                })
        }).map_err(|err| eprintln!("IO error: {:?}", err));
    tokio::run(task);
    */

    let task = tokio::fs::File::create("data.txt")
        .map(|file| {
            // let std_file = file.into_std();

            let mut buffer = [0; BUF_SIZE as usize ];
            let std_file = file.into_std();
            let mut handle = std_file.take(BUF_SIZE);
            let _r = handle.read(&mut buffer);

            println!("buffer: ");

            // do something with the std::fs::File
        }).map_err(|err| eprintln!("IO error: {:?}", err));

    tokio::run(task);
}

lazy_static! {
    static ref FREQ_LIMIT: RwLock<HashMap<i64, i64>> = { // channel_id => last_preload_time
        RwLock::new(HashMap::new())
    };
}

fn test() {
    {
        let mut freq_limit = FREQ_LIMIT.write().unwrap();
        freq_limit.insert(1, 5);
    }

    const channel_id: i32 = 1;
    let last_preload_time = *FREQ_LIMIT.read().unwrap().get(&1).unwrap_or(&0);

    // let now = lib_util::get_current_time();


    let can_preload = true;

    if can_preload {
        let mut freq_limit = FREQ_LIMIT.write().unwrap();
        freq_limit.insert(1, 5);
    }
}

lazy_static! {
    static ref PROFILING_RUNTIME: runtime::Runtime = runtime::Builder::new()
        .core_threads(1)
        .blocking_threads(10)
        .keep_alive(Some(Duration::from_secs(60)))
        .name_prefix("t:profiling_worker-")
        .after_start(|| {
        })
        .before_stop(|| {
        })
        .build()
        .expect("Initialize profiling runtime failed");

    static ref TIME_COUNT: RwLock<i32> = {
        RwLock::new(0)
    };
}

fn test_interval() {
    let interval = Duration::from_millis(1000);
    let task = Interval::new_interval(interval)
        .for_each(move |_| {
            // let my_time = Instant::now();
            // println!("task : {:?}", my_time);
            
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
                println!("execute cpu monitor failed: {:?}", e);
            }
            Ok(())
        });
    
    
    //let handler = thread::spawn(task);
    // handler.join().unwrap();
    
    PROFILING_RUNTIME.executor().spawn(task);
}

fn main() -> io::Result<()> {
    test_interval();
    let sleep_duration = Duration::from_secs(60*60*1000);
    thread::sleep(sleep_duration);

    test();
    test_thread();

    test_aync_file();

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