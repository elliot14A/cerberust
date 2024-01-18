#![allow(dead_code)]
pub(crate) mod account;
pub(crate) mod user;

use once_cell::sync::Lazy;
use surrealdb::{engine::remote::http::Client, Surreal};

pub(crate) static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
pub struct DatabaseImpl;

// #[async_trait]
// impl Database for DatabaseImpl {
//     async fn new() -> Self {
//         println!("surreadb impl");
//         Self
//     }
// }
