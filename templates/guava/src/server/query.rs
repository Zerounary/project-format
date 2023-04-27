use axum::{Json, extract::Path};
use serde_json::Value;

use super::{
    api::commands::{AppResult, Resp},
    auth::GuavaSession,
};

pub async fn query_list(
    Json(mut params): Json<Value>,
    Path(sql_name): Path<String>,
    {{#if auth}}
    GuavaSession::FoundUser((user, service)): GuavaSession,
    {{/if}}
) -> AppResult<Value> {
    {{#if auth}}
    params.as_object_mut().unwrap().insert("user".to_string(), serde_json::to_value(user).unwrap());
    {{/if}}
    let res = service.repo.query_list(&service.db, sql_name, params).await;
    Resp::ok(res)
}