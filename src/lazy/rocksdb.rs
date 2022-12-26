use once_cell::sync::OnceCell;
use surrealdb::{embedded::Db, storage::RocksDb, Surreal};

pub static DB: OnceCell<Surreal<Db>> = OnceCell::new();

pub async fn init_db_connection() -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<RocksDb>("temp.db").await.unwrap())
        .map_err(|_e| "init db error".to_string())
}
