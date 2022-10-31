use std::env;
use lazy_static::lazy_static;
use deadpool::managed::{Pool};
use deadpool_redis::{ Config, Runtime, Manager, Connection};

lazy_static! {
    // connect database
  pub static ref REDIS_URL: String =
      env::var("REDIS_URL").expect("No REDIS_URL provided");
}

pub fn init_redis() -> Pool<Manager, Connection> {
  let mut cfg = Config::from_url(REDIS_URL.as_str());
  let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
  pool
}