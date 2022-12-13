#[macro_export]
macro_rules! delete {
    ($model:ident) => {
        paste::paste! {
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _delete_thing>](resource: String) -> anyhow::Result<()> {
                let db = DB.get().unwrap();
                Ok(db.delete((stringify!($model),resource).into_thing()).await.unwrap())
            }
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _delete_json>](resource: String) -> anyhow::Result<()> {
                let db = DB.get().unwrap();
                Ok(db.delete(resource.into_object()).await.unwrap())
            }
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _delete_array>](
                resource: Vec<String>,
                range: Option<StringRange>,
            ) -> anyhow::Result<()> {
                let db = DB.get().unwrap();
                if let Some(range) = range {
                    Ok(db.delete(resource.into_array()).range(range).await.unwrap())
                } else {
                    Ok(db.delete(resource.into_array()).await.unwrap())
                }
            }
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _delete_table>](
                resource: String,
                range: Option<StringRange>,
            ) -> anyhow::Result<()> {
                let db = DB.get().unwrap();
                if let Some(range) = range {
                    Ok(db.delete(resource.into_table()).range(range).await.unwrap())
                } else {
                    Ok(db.delete(resource.into_table()).await.unwrap())
                }
            }
        }
    };
}
#[cfg(test)]
#[allow(dead_code)]
mod test {
    use crate::api::dependencies::*;
    #[derive(Serialize, Deserialize, Clone)]
    pub struct User {
        id: u64,
    }
    pub struct UserController;
    delete!(User);
}
