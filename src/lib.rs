#![feature(duration_as_u128)]

#[macro_use]
extern crate serde_derive;
use std::time::SystemTime;
use log::error;
use std::process;

pub mod args;
pub mod stats;
pub mod store;
pub mod serialize;

#[derive(Debug)]
pub enum SortMethod {
    Recent,
    Frequent,
    Frecent,
}

pub fn current_time_secs() -> f64 {
  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => n.as_millis() as f64 / 1000.0,
    Err(e) => {
      error!("invalid system time: {}", e);
      process::exit(1);
    }
  }
}

