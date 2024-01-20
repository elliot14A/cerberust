#![allow(dead_code)]
mod models;

pub use models::*;

use async_trait::async_trait;

#[async_trait]
pub trait DatabaseRepository: AccountRepository + UserRepository + Send + Sync + 'static {
    async fn new() -> Self;
    fn name(&self) -> String;
}
