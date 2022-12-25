use once_cell::sync::OnceCell;
use surrealdb::Surreal;
#[cfg(feature = "mem")]
use surrealdb::{embedded::Db, storage::Mem};
#[cfg(feature = "http")]
use surrealdb::{net::HttpClient, protocol::Https};
#[cfg(feature = "ws")]
use surrealdb::{net::WsClient, protocol::Ws};

#[cfg(feature = "mem")]
pub static DB: OnceCell<Surreal<Db>> = OnceCell::new();
#[cfg(feature = "http")]
pub static DB: OnceCell<Surreal<HttpClient>> = OnceCell::new();
#[cfg(feature = "ws")]
pub static DB: OnceCell<Surreal<WsClient>> = OnceCell::new();

#[cfg(feature = "mem")]
pub async fn init_db_connection() -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<Mem>(()).await.unwrap())
        .map_err(|_e| "init db error".to_string())
}
#[cfg(feature = "http")]
pub async fn init_db_connection(path: String) -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<Https>(path).await.unwrap())
        .map_err(|_e| "init db error".to_string())
}
#[cfg(feature = "ws")]
pub async fn init_db_connection(path: String) -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<Ws>(path).await.unwrap())
        .map_err(|_e| "init db error".to_string())
}
