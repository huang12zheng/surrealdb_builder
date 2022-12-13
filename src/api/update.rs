#[macro_export]
macro_rules! update {
    ($model:ident) => {
        paste::paste! {
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _update_thing>](resource: String, content: $model) -> anyhow::Result<$model > {
                let db = DB.get().unwrap();
                Ok(db.update((stringify!($model),resource).into_thing()).content(content).await.unwrap())
            }
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _update_json>](resource: String, content: $model) -> anyhow::Result<$model > {
                let db = DB.get().unwrap();
                Ok(db.update(resource.into_object()).content(content).await.unwrap())
            }
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _update_array>](
                resource: Vec<String>,
                range: Option<StringRange>,
                content: $model
            ) -> anyhow::Result<Vec<$model >> {
                let db = DB.get().unwrap();
                if let Some(range) = range {
                    Ok(db.update(resource.into_array()).range(range).content(content).await.unwrap())
                } else {
                    Ok(db.update(resource.into_array()).content(content).await.unwrap())
                }
            }
            #[tokio::main(flavor = "current_thread")]
            pub async fn [<$model:snake _update_table>](
                resource: String,
                range: Option<StringRange>,
                content: $model
            ) -> anyhow::Result<Vec<$model >> {
                let db = DB.get().unwrap();
                if let Some(range) = range {
                    Ok(db.update(resource.into_table()).range(range).content(content).await.unwrap())
                } else {
                    Ok(db.update(resource.into_table()).content(content).await.unwrap())
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
    update!(User);
}
