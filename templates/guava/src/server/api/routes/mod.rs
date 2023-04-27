pub mod root_routes;
{{#each tables}}
pub mod {{{name}}}_routes;
{{/each}}


use axum::Router;

use self::root_routes::root_routes;
{{#each tables}}
use self::{{{name}}}_routes::{{{name}}}_routes;
{{/each}}

pub fn api_routes() -> Router {
    Router::new()
        .nest("/", root_routes())
        {{#each tables}}
        .nest("/{{{name}}}", {{{name}}}_routes())
        {{/each}}
}