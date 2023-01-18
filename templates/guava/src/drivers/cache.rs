use moka::sync::Cache;
use std::{sync::Arc, collections::HashMap};
use crate::Redis;
use super::redis::init_redis;
use crate::{
    server::error::AppError,
    {{#each tables}}
    entities::{{this.name}}_bo::{{upperCamel this.name}}BO,
    {{/each}}
};

#[derive(Debug, Clone)]
pub enum ServiceResult {
    {{#each tables}}
    {{upperCamel this.name}}BO(Result<{{upperCamel this.name}}BO, AppError>),
    {{/each}}
}

{{#if redis_session}}
pub type ServiceCache = Redis;
{{else}}
pub type CacheCell = Arc<Cache<String, ServiceResult>>;
pub type ServiceCache = Arc<HashMap<String, CacheCell>>;
{{/if}}

#[macro_export]
macro_rules! cache_value {
    ($name:ident as Result<$type:ident, $err:ident>) => { {
        let value: Result<$type, $err> = match $name {
            ServiceResult::$type(v) => match v {
                Ok(bo) => Ok(bo),
                Err(e) => Err(e),
            },
            _ => Err(AppError::RepoError("cache_value! failed".to_string())),
        };
        value
    } };

    ($name:ident as $type:ident) => { {
        let value: Option<$type> = match $name {
            ServiceResult::$type(v) => Some(v.clone()),
            _ => None,
        };
        value
    } };
}


#[macro_export]
macro_rules! cache {
    ($self:ident($key:ident) -> Result<$bo:ident, $err:ident> $block:block) => {
        {{#if redis_session}}
        let table_name = crate::macros::repository::pure_name(stringify!($bo));
        let key  = if  let Some(user) = $self.user.clone() {
            format!("{}-{}-{}", user.tenant_id, table_name, $key.to_string())
        }else {
            $key.to_string()
        };
        let mut conn = $self.cache.get().await?;
        let result: Option<String> = conn.get(key.clone()).await?;
        match result {
            Some(json_str) => {
                serde_json::from_str(&json_str).unwrap()
            }
            None => {
                let result: Result<$bo, AppError> = $block;
                let result_str = serde_json::to_string(&result)?;
                conn.set(&key, result_str).await?;
                result
            }
        }
        {{else}}
        let table_name = crate::macros::repository::pure_name(stringify!($bo));
        let key  = if  let Some(user) = $self.user.clone() {
            format!("{}-{}", user.tenant_id, $key.to_string())
        }else {
            $key.to_string()
        };
        let cache = $self.cache.get(&table_name).unwrap();
        match cache.get(&key) {
            Some(e) => {
                let x = cache_value!(e as Result<$bo, $err>);
                x
            }
            None => {
                let result: Result<$bo, $err> = $block;
                cache.insert(key, ServiceResult::$bo(result.clone()));
                result
            }
        }
        {{/if}}
    };
}

#[macro_export]
macro_rules! cache_invalidate {
    ($self:ident($key:expr => $bo:ident)) => {
        {{#if redis_session}}
        let table_name = crate::macros::repository::pure_name(stringify!($bo));
        let key  = if  let Some(user) = $self.user.clone() {
            format!("{}-{}-{}", user.tenant_id, table_name, $key.to_string())
        }else {
            $key.to_string()
        };
        let mut conn = $self.cache.get().await?;
        conn.del(&key).await?;
        {{else}}
        let table_name = crate::macros::repository::pure_name(stringify!($bo));
        let key = $key.to_string();
        let cache = $self.cache.get(&table_name).unwrap();
        cache.invalidate(&key);
        {{/if}}
    }
}

pub fn init_cache() -> ServiceCache {
    {{#if redis_session}}
    init_redis()
    {{else}}
    Arc::new(HashMap::from([
        {{#each tables}}
        ("{{name}}".to_string(), Arc::new(Cache::new(10_000))),
        {{/each}}
    ]))
    {{/if}}
}