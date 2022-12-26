use once_cell::sync::OnceCell;
use surrealdb::{net::HttpClient, protocol::Https, Surreal};

pub static DB: OnceCell<Surreal<HttpClient>> = OnceCell::new();

pub async fn init_db_connection(path: String) -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<Https>(path).await.unwrap())
        .map_err(|_e| "init db error".to_string())
}
