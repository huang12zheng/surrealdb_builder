use once_cell::sync::OnceCell;
use surrealdb::{net::WsClient, protocol::Ws, Surreal};

pub static DB: OnceCell<Surreal<WsClient>> = OnceCell::new();

pub async fn init_db_connection(path: String) -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<Ws>(path).await.unwrap())
        .map_err(|_e| "init db error".to_string())
}
