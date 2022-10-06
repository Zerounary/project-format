use moka::sync::Cache;
use std::sync::Arc;

use crate::{
    {{#each tables}}
    entities::{{this.name}}_bo::{{upperCamel this.name}}BO, service::{{this.name}}_service::{{upperCamel this.name}}RepoError,
    {{/each}}
};

#[derive(Debug, Clone)]
pub enum ServiceResult {
    {{#each tables}}
    {{upperCamel this.name}}BO(Result<{{upperCamel this.name}}BO, {{upperCamel this.name}}RepoError>),
    {{/each}}
}

pub type ServiceCache = Arc<Cache<i64, ServiceResult>>;

#[macro_export]
macro_rules! cache_value {
    ($name:ident as Result<$type:ident, $err:ident>) => { {
        let value: Result<$type, $err> = match $name {
            ServiceResult::$type(v) => match v {
                Ok(bo) => Ok(bo),
                Err(e) => Err(e),
            },
            _ => Err($err::NotFound),
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
        match $self.cache.get(&$key) {
            Some(e) => {
                let x = cache_value!(e as Result<$bo, $err>);
                x
            }
            None => {
                let result: Result<$bo, $err> = $block;
                $self
                    .cache
                    .insert($key, ServiceResult::$bo(result.clone()));
                result
            }
        }
    };
}
