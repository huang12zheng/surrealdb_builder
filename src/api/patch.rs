use super::dependencies::IntoValue;
use surrealdb::param::PatchOp;
pub struct Add {
    pub path: String,
    pub value: String,
}
pub struct Remove {
    pub path: String,
}
pub struct Replace {
    pub path: String,
    pub value: String,
}

pub enum InnerOp {
    Add(Add),
    Remove(Remove),
    Replace(Replace),
}

pub trait IntoPatchOp {
    fn into_patch_op(self) -> PatchOp;
}

impl IntoPatchOp for InnerOp {
    fn into_patch_op(self) -> PatchOp {
        match self {
            InnerOp::Add(Add { path, value }) => PatchOp::add(&path, value.into_value()),
            InnerOp::Remove(Remove { path }) => PatchOp::remove(&path),
            InnerOp::Replace(Replace { path, value }) => {
                PatchOp::replace(&path, value.into_value())
            }
        }
    }
}

#[macro_export]
macro_rules! patch {
    ($model:ident) => {
        paste::paste! {
            impl [< $model Controller >] {
                pub async fn patch_thing(&self, resource: String, content: InnerOp) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.update((stringify!($model),resource).into_thing()).patch(content.into_patch_op()).await.unwrap())
                }
                pub async fn patch_json(&self, resource: String, content: InnerOp) -> anyhow::Result<$model > {
                    let db = DB.get().unwrap();
                    Ok(db.update(resource.into_object()).patch(content.into_patch_op()).await.unwrap())
                }

                pub async fn patch_array(
                    &self,
                    resource: Vec<String>,
                    range: Option<StringRange>,
                    content: InnerOp
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.update(resource.into_array()).range(range).patch(content.into_patch_op()).await.unwrap())
                    } else {
                        Ok(db.update(resource.into_array()).patch(content.into_patch_op()).await.unwrap())
                    }
                }

                // pub async fn patch_edges(
                //     &self,
                //     resource: Edges,
                //     range: Option<StringRange>,
                //     content: InnerOp
                // ) -> anyhow::Result<Vec<$model >> {
                //     let db = DB.get().unwrap();
                //     if let Some(range) = range {
                //         Ok(db.update(resource).range(range).patch(content.into_patch_op()).await.unwrap())
                //     } else {
                //         Ok(db.update(resource).patch(content.into_patch_op()).await.unwrap())
                //     }
                // }

                pub async fn patch_table(
                    &self,
                    resource: Table,
                    range: Option<StringRange>,
                    content: InnerOp
                ) -> anyhow::Result<Vec<$model >> {
                    let db = DB.get().unwrap();
                    if let Some(range) = range {
                        Ok(db.update(resource).range(range).patch(content.into_patch_op()).await.unwrap())
                    } else {
                        Ok(db.update(resource).patch(content.into_patch_op()).await.unwrap())
                    }
                }
            }
        }
    };
}
#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;
    use crate::api::dependencies::*;
    #[derive(Serialize, Deserialize, Clone)]
    pub struct User {
        id: u64,
    }
    pub struct UserController;
    patch!(User);
}
