// #[macro_export]
// macro_rules! merge {
//     ($model:ident) => {
//         paste::paste! {
//             pub struct [< $model Controller >];

//             impl [< $model Controller >] {
//                 pub async fn merge_thing(&self, resource: Thing, content: $model) -> anyhow::Result<$model > {
//                     let db = DB.get().unwrap();
//                     Ok(db.merge(resource).merge(content).await.unwrap())
//                 }
//                 pub async fn merge_json(&self, resource: String, content: $model) -> anyhow::Result<$model > {
//                     let db = DB.get().unwrap();
//                     Ok(db.merge(resource.into_object()).merge(content).await.unwrap())
//                 }

//                 pub async fn merge_array(
//                     &self,
//                     resource: Vec<String>,
//                     range: Option<StringRange>,
//                     content: $model
//                 ) -> anyhow::Result<Vec<$model >> {
//                     let db = DB.get().unwrap();
//                     if let Some(range) = range {
//                         Ok(db.merge(resource.into_array()).range(range).merge(content).await.unwrap())
//                     } else {
//                         Ok(db.merge(resource.into_array()).merge(content).await.unwrap())
//                     }
//                 }

//                 pub async fn merge_edges(
//                     &self,
//                     resource: Edges,
//                     range: Option<StringRange>,
//                     content: $model
//                 ) -> anyhow::Result<Vec<$model >> {
//                     let db = DB.get().unwrap();
//                     if let Some(range) = range {
//                         Ok(db.merge(resource).range(range).merge(content).await.unwrap())
//                     } else {
//                         Ok(db.merge(resource).merge(content).await.unwrap())
//                     }
//                 }

//                 pub async fn merge_table(
//                     &self,
//                     resource: Table,
//                     range: Option<StringRange>,
//                     content: $model
//                 ) -> anyhow::Result<Vec<$model >> {
//                     let db = DB.get().unwrap();
//                     if let Some(range) = range {
//                         Ok(db.merge(resource).range(range).merge(content).await.unwrap())
//                     } else {
//                         Ok(db.merge(resource).merge(content).await.unwrap())
//                     }
//                 }
//             }
//         }
//     };
// }
// #[cfg(test)]
// mod test {
//     use crate::api::dependencies::*;
//     #[derive(Serialize, Deserialize, Clone)]
//     pub struct User {
//         id: u64,
//     }
//     merge!(User);
// }
