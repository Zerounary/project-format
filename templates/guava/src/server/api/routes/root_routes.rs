use crate::server::auth::{check_auth, login, logout};
use axum::{routing::get, Router};

pub fn root_routes() -> Router {
    Router::new()
        {{#if auth}}
        .route("/login", get(login))
        .route("/logout", get(logout))
        .route("/auth", get(check_auth))
        {{/if}}
}
