pub mod create;
pub mod delete;
pub mod dependencies;
pub mod merge;
pub mod patch;
pub mod select;
pub mod update;

use once_cell::sync::OnceCell;
use surrealdb::{embedded::Db, storage::Mem, Surreal};

pub struct IDb;
pub static DB: OnceCell<Surreal<Db>> = OnceCell::new();

#[tokio::main(flavor = "current_thread")]
pub async fn init_db() -> Result<(), std::string::String> {
    DB.set(Surreal::connect::<Mem>(()).await.unwrap())
        .map_err(|e| format!("init db error"))
}
