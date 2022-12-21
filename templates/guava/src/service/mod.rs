use std::{sync::{Arc}, collections::HashMap};

use moka::sync::Cache;

use crate::{repository::Repository, drivers::{db::DB, cache::{ServiceCache}}, Redis};
use crate::{server::auth::SessionUser};

{{#each tables}}
pub mod {{name}}_service;
{{/each}}

pub struct Service {
    pub repo: Repository,
    pub db: DB, // 为了调用事物
    user: Option<SessionUser>,
    cache: Redis 
}


impl Service {
    pub fn new_scope(db: DB, user: SessionUser, cache: Redis) -> Service {
        let repo = Repository::new(Some(user.clone()));
        Service {
            db,
            repo,
            cache,
            user: Some(user),
        }
    }

    pub fn new(db: DB, cache: Redis) -> Service {
        let repo = Repository::new(None);
        Service {
            db,
            repo,
            cache,
            user: None,
        }
    }
}