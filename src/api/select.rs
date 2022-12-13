#[macro_export]
macro_rules! select {
    ($model:ident) => {
        paste::paste! {
            impl [< $model Controller >] {
                #[tokio::main(flavor = "current_thread")]
                pub async fn select_thing(&self, resource: String) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.select((stringify!($model),resource).into_thing()).await.unwrap())
                }
                #[tokio::main(flavor = "current_thread")]
                pub async fn select_json(&self, resource: String) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.select(resource.into_object()).await.unwrap())
                }
                #[tokio::main(flavor = "current_thread")]
                pub async fn select_array(
                    &self,
                    resource: Vec<String>,
                    range: Option<StringRange>,
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.select(resource.into_array()).range(range).await.unwrap())
                    } else {
                        Ok(db.select(resource.into_array()).await.unwrap())
                    }
                }
                #[tokio::main(flavor = "current_thread")]
                pub async fn select_edges(
                    &self,
                    resource: EdgesMirror,
                    range: Option<StringRange>,
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.select(resource.into_edges()).range(range).await.unwrap())
                    } else {
                        Ok(db.select(resource.into_edges()).await.unwrap())
                    }
                }
                #[tokio::main(flavor = "current_thread")]
                pub async fn select_table(
                    &self,
                    resource: String,
                    range: Option<StringRange>,
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.select(resource.into_table()).range(range).await.unwrap())
                    } else {
                        Ok(db.select(resource.into_table()).await.unwrap())
                    }
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
    select!(User);
}
