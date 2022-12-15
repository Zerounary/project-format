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
pub type AppListResult<T> = AppResult<Items<Vec<T>>>;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct Resp<T> {
    status: i32,
    msg: String,
    data: Option<T>,
}

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct Empty;

impl<T> Resp<T> {
    pub fn page(count: Option<i64>, rows: Vec<T>) -> AppListResult<T> {
        Ok(Json(Resp{
            status: 0,
            msg: "ok".to_string(),
            data: Some(Items{
                count,
                rows
            }),
        }))
    }
    pub fn ok(data: T) -> AppResult<T> {
        Ok(Json(Self {
            status: 0,
            msg: "ok".to_string(),
            data: Some(data),
        }))
    }
}

pub fn resp_err(status: i32, msg: String) -> Json<Resp<Empty>> {
    Json(Resp {
        status,
        msg,
        data: None,
    })
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    page: Option<i64>,
    perPage: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Items<T> {
    count: Option<i64>,
    rows: T,
}