#[macro_export]
macro_rules! select {
    ($model:ident) => {
        paste::paste! {
            pub struct [< $model Controller >];

            impl [< $model Controller >] {
                pub async fn select_thing(&self, resource: Thing) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.select(resource).await.unwrap())
                }
                pub async fn select_json(&self, resource: String) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.select(resource.into_object()).await.unwrap())
                }

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

                pub async fn select_edges(
                    &self,
                    resource: Edges,
                    range: Option<StringRange>,
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.select(resource).range(range).await.unwrap())
                    } else {
                        Ok(db.select(resource).await.unwrap())
                    }
                }

                pub async fn select_table(
                    &self,
                    resource: Table,
                    range: Option<StringRange>,
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.select(resource).range(range).await.unwrap())
                    } else {
                        Ok(db.select(resource).await.unwrap())
                    }
                }
            }
        }
    };
}
#[cfg(test)]
mod test {
    use crate::api::dependencies::*;
    #[derive(Serialize, Deserialize, Clone)]
    pub struct User {
        id: u64,
    }
    select!(User);
}
