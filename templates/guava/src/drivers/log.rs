use lazy_static::lazy_static;
use std::env;

use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;

lazy_static! {
    pub static ref LOG_ENABLE: String =
        env::var("LOG_ENABLE").unwrap_or(String::from("1"));

    pub static ref LOG_FILE: String =
        env::var("LOG_FILE").unwrap_or(String::from("logs/"));

    pub static ref LOG_SIZE: String =
        env::var("LOG_SIZE").unwrap_or(String::from("1"));
}

pub fn init_log() {
  if LOG_ENABLE.as_str().parse::<i64>().unwrap() == 1 {
    println!("log enable");
    fast_log::init(fast_log::Config::new().file_split(
        LOG_FILE.as_str(),
        LogSize::MB(LOG_SIZE.as_str().parse::<usize>().unwrap()),
        RollingType::All,
        LogPacker {},
    ))
    .unwrap();
  }
}
