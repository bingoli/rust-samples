extern crate tokio;
extern crate chrono;
#[macro_use]
extern crate lazy_static;

pub mod perf_test;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
