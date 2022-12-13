#[macro_export]
macro_rules! select {
    ($model:ident) => {
        paste::paste! {
        #[tokio::main(flavor = "current_thread")]
        pub async fn [<$model:snake _select_thing>](resource: String) -> anyhow::Result<$model > {
            let db = DB.get().unwrap();
            Ok(db.select((stringify!($model),resource).into_thing()).await.unwrap())
        }
        #[tokio::main(flavor = "current_thread")]
        pub async fn [<$model:snake _select_json>](resource: String) -> anyhow::Result<$model > {
            let db = DB.get().unwrap();
            Ok(db.select(resource.into_object()).await.unwrap())
        }
        #[tokio::main(flavor = "current_thread")]
        pub async fn [<$model:snake _select_array>](
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
        pub async fn [<$model:snake _select_edges>](
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
        pub async fn [<$model:snake _select_table>](
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
    select!(User);
}
