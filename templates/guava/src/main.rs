pub mod drivers;
pub mod entities;
pub mod repository;
pub mod server;
pub mod service;
pub mod macros;

use crate::{
    drivers::{db::{init_db, migrate}, log::{init_log}, redis::init_redis},
    server::api::commands::{
        {{#each tables}}
        {{this.name}}_controller::{find_{{ this.name }}_page, find_{{ this.name }}_list, delete_{{ this.name }}_ids, find_{{ this.name }}_by_id, update_{{ this.name }}, update_{{ this.name }}_opt, create_{{ this.name }}, create_{{ this.name }}_batch},
        {{/each}}
    },
    service::Service,
};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use deadpool::managed::{Pool};
use deadpool_redis::{ Manager, Connection};
use tower_http::{trace::TraceLayer};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::signal;

pub struct AppState {
    service: Service,
    redis: Pool<Manager, Connection>,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    init_log();

    migrate().await;

    let db = init_db();

    let redis = init_redis();

    // Inject a `AppState` into our handlers via a trait object. This could be
    // the live implementation or just a mock for testing.
    let service = Arc::new(AppState {
        service: Service::new(db),
        redis,
    });

    // build our application with a route
    let app = Router::new()

        {{#each tables}}
        .route("/api/{{ this.name }}/list", post(find_{{ this.name }}_list))
        .route("/api/{{ this.name }}/page", post(find_{{ this.name }}_page))
        .route(
            "/api/{{ this.name }}/:id",
            get(find_{{ this.name }}_by_id).delete(delete_{{ this.name }}_ids).patch(update_{{ this.name }}_opt).put(update_{{ this.name }}),
        )
        .route("/api/{{ this.name }}", post(create_{{ this.name }}))
        .route("/api/{{ this.name }}/batch", post(create_{{ this.name }}_batch))
        {{/each}}
        .merge(axum_extra::routing::SpaRouter::new("/assets", "dist/assets").index_file("../index.html")) // 静态页面直接复制dist目录到guava同级目录 会匹配首页
        .layer(Extension(service))
        .layer(TraceLayer::new_for_http());

    // run it
    let port = env::var("PORT").unwrap_or_default().parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    anyhow::Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}