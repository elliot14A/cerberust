use std::sync::Arc;

use axum::{Extension, Router};
use diesel_async::pooled_connection::{bb8, AsyncDieselConnectionManager};
use hyper::Method;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

use tracing::info;

use crate::{api::init_routes, logger::logger};

pub async fn build_http_server() -> anyhow::Result<(TcpListener, Router)> {
    let (config, default) = crate::config::Config::load()?;

    let default_http_port = config.config.port.clone();
    let default_addr = "0.0.0.0".to_string();
    let default_addr = format!("{}:{}", default_addr, default_http_port);

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(tower_http::cors::Any)
        .allow_credentials(false);

    let app = Router::new().layer(cors);

    // connect to database
    let database_url = config.config.database_url.clone();

    let db_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&database_url);
    let pool = bb8::Pool::builder().build(db_config).await?;

    let routes = init_routes(pool.clone());

    let app = app.nest("/api", routes).layer(logger());

    if !default {
        info!("🛠 Creating resources and roles from cerberust.toml");
        let mut conn = pool.get().await?;
        let root_user_id = config.create_root_user(&mut conn).await?;
        if root_user_id.is_none() {
            info!("🚫 Root user not created. Skipping creating resources! ")
        } else {
            config
                .create_resources(root_user_id.unwrap(), &mut conn)
                .await?;
        }
        config.create_roles(&mut conn).await?;
    } else {
        info!("🛠 Cannot find cerberust.toml file. Using default configuration.");
    }

    // build smtp service
    let smtp = crate::utils::smtp::SmtpService::new(config.config);

    let app = app
        .layer(CookieManagerLayer::new())
        .layer(Extension(Arc::new(smtp)));

    let listner = TcpListener::bind(default_addr.clone()).await?;
    info!("🚀 Listening on {}", default_addr);
    Ok((listner, app))
}
