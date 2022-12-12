{{#each tables}}
pub mod {{name}}_controller;
{{/each}}

use crate::{server::error::AppError, AppState, service::Service};
use axum::{response::Json, Extension};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use smart_default::SmartDefault;
use rbatis::rbdc::{decimal::Decimal, date::Date};

// TODO 用 serde_json::Value 来接所有不知道类型的，又要存起来的数据。 也可以看是否可以用Box

// TODO 用 type 别名来收缩复杂的类型

pub type State = Extension<Arc<Service>>;

pub type AppResult<T> = Result<Json<Resp<T>>, AppError>;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct Resp<T> {
    code: i32,
    msg: String,
    total: Option<i64>,
    data: Option<T>,
}

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct Empty;

impl<T> Resp<T> {
    pub fn page(total: i64, data: T) -> AppResult<T> {
        Ok(Json(Self {
            code: 0,
            msg: "ok".to_string(),
            data: Some(data),
            total: Some(total)
        }))
    }
    pub fn ok(data: T) -> AppResult<T> {
        Ok(Json(Self {
            code: 0,
            msg: "ok".to_string(),
            data: Some(data),
            total: None
        }))
    }
}

pub fn resp_err(code: i32, msg: String) -> Json<Resp<Empty>> {
    Json(Resp {
        code,
        msg,
        data: None,
        total: None,
    })
}