#![allow(dead_code)]
mod actions;
mod api;
mod error;
mod extractors;
mod logger;
mod models;
mod schema;
mod server;
mod utils;

use server::build_http_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let (listner, app) = build_http_server().await?;
    axum::serve(listner, app).await?;
    Ok(())
}
