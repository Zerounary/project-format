use std::{sync::{Arc}, collections::HashMap};

use moka::sync::Cache;

use crate::{repository::Repository, drivers::{db::DB, cache::{ServiceCache}}};
use crate::{server::auth::SessionUser};

{{#each tables}}
pub mod {{name}}_service;
{{/each}}

#[derive(Debug)]
pub struct Service {
    pub repo: Repository,
    pub db: DB, // 为了调用事物
    user: Option<SessionUser>,
    cache: Arc<HashMap<String, ServiceCache>>
}

fn init_cache() -> Arc<HashMap<String, ServiceCache>>{
    Arc::new(HashMap::from([
        {{#each tables}}
        ("{{name}}".to_string(), Arc::new(Cache::new(10_000))),
        {{/each}}
    ]))
}

impl Service {
    pub fn new_scope(db: DB, user: SessionUser) -> Service {
        let repo = Repository::new();
        Service {
            db,
            repo,
            user: Some(user),
            cache: init_cache()
        }
    }

    pub fn new(db: DB ) -> Service {
        let repo = Repository::new();
        Service {
            db,
            repo,
            user: None,
            cache: init_cache()
        }
    }
}