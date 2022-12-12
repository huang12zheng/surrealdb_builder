#[macro_export]
macro_rules! create {
    ($model:ident) => {
        paste::paste! {
            pub struct [< $model Controller >];

            impl [< $model Controller >] {
                pub async fn create_thing(&self, resource: String, content: $model) -> anyhow::Result<$model> {
                    let db = DB.get().unwrap();
                    Ok(db.create((stringify!($model),resource).into_thing()).content(content).await.unwrap())
                }
                pub async fn create_json(&self, resource: String, content: $model) -> anyhow::Result<$model> {
                    let db = DB.get().unwrap();
                    Ok(db.create(resource.into_object()).content(content).await.unwrap())
                }

                pub async fn create_array(
                    &self,
                    resource: Vec<String>,
                    content: $model
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    Ok(db.create(resource.into_array()).content(content).await.unwrap())
                }

                // pub async fn create_edges(
                //     &self,
                //     resource: Edges,
                //     range: Option<StringRange>,
                //     content: $model
                // ) -> anyhow::Result<Vec<$model >> {
                //     let db = DB.get().unwrap();
                //     Ok(db.create(resource).content(content).await.unwrap())
                // }

                pub async fn create_table(
                    &self,
                    resource: Table,
                    content: $model
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    Ok(db.create(resource).content(content).await.unwrap())
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
    create!(User);
}
