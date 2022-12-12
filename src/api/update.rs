#[macro_export]
macro_rules! update {
    ($model:ident) => {
        paste::paste! {
            pub struct [< $model Controller >];

            impl [< $model Controller >] {
                pub async fn update_thing(&self, resource: Thing, content: $model) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.update(resource).content(content).await.unwrap())
                }
                pub async fn update_json(&self, resource: String, content: $model) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.update(resource.into_object()).content(content).await.unwrap())
                }

                pub async fn update_array(
                    &self,
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

                pub async fn update_edges(
                    &self,
                    resource: Edges,
                    range: Option<StringRange>,
                    content: $model
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.update(resource).range(range).content(content).await.unwrap())
                    } else {
                        Ok(db.update(resource).content(content).await.unwrap())
                    }
                }

                pub async fn update_table(
                    &self,
                    resource: Table,
                    range: Option<StringRange>,
                    content: $model
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.update(resource).range(range).content(content).await.unwrap())
                    } else {
                        Ok(db.update(resource).content(content).await.unwrap())
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
    update!(User);
}
