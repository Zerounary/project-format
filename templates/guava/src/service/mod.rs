use std::{sync::{Arc}, collections::HashMap};

use moka::sync::Cache;

use crate::{repository::Repository, drivers::{db::DB, cache::{ServiceCache}}};

{{#each tables}}
pub mod {{name}}_service;
{{/each}}

#[derive(Debug)]
pub struct Service {
    pub repo: Repository,
    pub db: DB, // 为了调用事物
    cache: Arc<HashMap<String, ServiceCache>>
}

impl Service {
    pub fn new(db: DB ) -> Service {
        let repo = Repository::new();
        let cache = Arc::new(HashMap::from([
            {{#each tables}}
            ("{{name}}".to_string(), Arc::new(Cache::new(10_000))),
            {{/each}}
        ]));
        Service {
            db,
            repo,
            cache
        }
    }
}