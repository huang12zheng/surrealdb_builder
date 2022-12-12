use once_cell::sync::OnceCell;
use surrealdb::{embedded::Db, storage::Mem, Surreal};

// pub struct IDb;
pub static DB: OnceCell<Surreal<Db>> = OnceCell::new();

pub async fn init_db() -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<Mem>(()).await.unwrap())
        .map_err(|_e| "init db error".to_string())
}
