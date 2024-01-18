#[cfg(feature = "default")]
use surrealdb_driver::SurrealDriver as DatabaseImpl;

#[cfg(feature = "postgres")]
use postgres_driver::DatabaseImpl;

use repositories::Database;
#[tokio::main]
async fn main() {
    // let _ = DatabaseImpl::new().await;
    println!("hello world");
}
