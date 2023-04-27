use crate::server::{auth::{check_auth, login, logout}, query::query_list};
use axum::{routing::{get, post}, Router};

pub fn root_routes() -> Router {
    Router::new()
        {{#if auth}}
        .route("/login", get(login))
        .route("/logout", get(logout))
        .route("/auth", get(check_auth))
        {{/if}}
        .route("/ql/:sql_name", post(query_list))
}
