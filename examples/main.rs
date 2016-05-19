extern crate zlog;
#[macro_use] extern crate log;
use std::env;

pub fn main() {
    let log_spec = env::var("ZLOG").unwrap_or("main=INFO".to_string());
    zlog::init(&log_spec);
    info!("Hello, World");
    println!("did it work?");
}
