#![allow(dead_code)]

use server::build_http_server;
mod api;
mod logger;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let (listner, app) = build_http_server().await?;
    axum::serve(listner, app).await?;
    Ok(())
}
