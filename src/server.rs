use std::sync::Arc;

use axum::{Extension, Router};
use diesel_async::pooled_connection::{bb8, AsyncDieselConnectionManager};
use hyper::Method;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

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

    let app = Router::new().layer(cors).layer(logger());

    // connect to database
    let database_url = std::env::var("DATABASE_URL").unwrap_or(String::from(
        "postgres://postgres:postgres@localhost/cerberust",
    ));
    // let connection = AsyncPgConnection::establish(&database_url).await?;
    info!("🚀 Connected to Postgres");

    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&database_url);
    let pool = bb8::Pool::builder().build(config).await?;

    let routes = init_routes(pool);

    let app = app.nest("/api", routes);

    // build smtp service
    let smtp = crate::utils::smtp::SmtpService::new();
    info!("🚀 Conntected to SMTP Server");

    let app = app
        .layer(CookieManagerLayer::new())
        .layer(Extension(Arc::new(smtp)));

    let listner = TcpListener::bind(default_addr.clone()).await?;
    info!("🚀 Listening on {}", default_addr);
    Ok((listner, app))
}
