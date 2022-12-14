use deadpool::managed::Pool;
use deadpool_redis::{Config, Connection, Manager, Runtime};
use lazy_static::lazy_static;
use std::env;

use super::session::{GuavaSessionStore};

pub type Redis = Pool<Manager, Connection>;

lazy_static! {
    // connect database
  pub static ref REDIS_URL: String =
      env::var("REDIS_URL").expect("No REDIS_URL provided");
}

pub fn init_redis() -> Redis {
    let mut cfg = Config::from_url(REDIS_URL.as_str());
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    pool
}

pub fn init_redis_session_store() -> GuavaSessionStore {
    GuavaSessionStore::new()
}