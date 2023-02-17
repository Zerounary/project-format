use axum::{
    routing::{get, post},
    Router,
};

use crate::server::api::commands::{{{name}}}_controller::{
    create_{{{name}}}, create_{{{name}}}_batch, delete_{{{name}}}_ids, find_{{{name}}}_by_id,
    find_{{{name}}}_list, find_{{{name}}}_page, update_{{{name}}}, update_{{{name}}}_opt,
};

pub fn {{{name}}}_routes() -> Router {
    Router::new()
        .route("/list", get(find_{{{name}}}_list))
        .route("/page", get(find_{{{name}}}_page))
        .route(
            "/:id",
            get(find_{{{name}}}_by_id)
                .delete(delete_{{{name}}}_ids)
                .patch(update_{{{name}}}_opt)
                .put(update_{{{name}}}),
        )
        .route("/", post(create_{{{name}}}))
        .route("/batch", post(create_{{{name}}}_batch))
}
