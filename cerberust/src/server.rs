use std::sync::Arc;

use axum::{Extension, Router};
use hyper::Method;
use repositories::DatabaseRepository;
use tokio::net::TcpListener;

#[cfg(feature = "surrealdb")]
use surrealdb_driver::SurrealDriver as DatabaseDriver;

#[cfg(feature = "postgres")]
use postgres_driver::PostgresDriver as DatabaseDriver;

use tracing::info;

use crate::{api::init_routes, logger::logger};

pub async fn build_http_server() -> anyhow::Result<(TcpListener, Router)> {
    let default_http_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let default_addr = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let default_addr = format!("{}:{}", default_addr, default_http_port);

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(tower_http::cors::Any)
        .allow_credentials(false);

    let database_repositoy = DatabaseDriver::new().await;
    let app = init_routes::<DatabaseDriver>();

    let app = app
        .layer(cors)
        .layer(logger())
        .layer(Extension(Arc::new(database_repositoy)));

    let listner = TcpListener::bind(default_addr.clone()).await?;
    info!("🚀 Listening on {}", default_addr);
    Ok((listner, app))
}
