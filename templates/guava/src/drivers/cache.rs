use moka::sync::Cache;
use std::{sync::Arc, collections::HashMap};

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

pub type ServiceCache = Arc<Cache<String, ServiceResult>>;

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
    };
}

#[macro_export]
macro_rules! cache_invalidate {
    ($self:ident($key:expr => $bo:ident)) => {
        let table_name = crate::macros::repository::pure_name(stringify!($bo));
        let key = $key.to_string();
        let cache = $self.cache.get(&table_name).unwrap();
        cache.invalidate(&key);
    }
}

pub fn init_cache() -> Arc<HashMap<String, ServiceCache>>{
    Arc::new(HashMap::from([
        {{#each tables}}
        ("{{name}}".to_string(), Arc::new(Cache::new(10_000))),
        {{/each}}
    ]))
}