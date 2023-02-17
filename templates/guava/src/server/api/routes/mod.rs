pub mod auth_routes;
{{#each tables}}
pub mod {{{name}}}_routes;
{{/each}}


use axum::Router;

use self::auth_routes::auth_routes;
{{#each tables}}
use self::{{{name}}}_routes::{{{name}}}_routes;
{{/each}}

pub fn api_routes() -> Router {
    Router::new()
        {{#if auth}}
        .nest("/", auth_routes())
        {{/if}}
        {{#each tables}}
        .nest("/{{{name}}}", {{{name}}}_routes())
        {{/each}}
}