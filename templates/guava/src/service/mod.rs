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
    cache: ServiceCache 
}


impl Service {
    pub fn new_scope(db: DB, user: SessionUser, cache: ServiceCache) -> Service {
        let repo = Repository::new(Some(user.clone()));
        Service {
            db,
            repo,
            cache,
            user: Some(user),
        }
    }

    pub fn new(db: DB, cache: ServiceCache) -> Service {
        let repo = Repository::new(None);
        Service {
            db,
            repo,
            cache,
            user: None,
        }
    }


    pub async fn conn_begin(&self) -> Result<rbatis::executor::RBatisTxExecutorGuard, rbatis::rbdc::Error> {
        let conn = self.db.acquire_begin().await?;
        let conn = conn.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
            }
        });
        Ok(conn)
    }

    pub async fn conn(&self) -> Result<rbatis::executor::RBatisConnExecutor, rbatis::rbdc::Error> {
        let conn = self.db.acquire().await?;
        Ok(conn)
    }
}