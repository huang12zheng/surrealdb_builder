use surrealdb::sql::Array;

pub trait IntoArray {
    fn into_array(self) -> Array;
}

impl IntoArray for Vec<String> {
    fn into_array(self) -> Array {
        Array(self.into_iter().map(|e| e.into()).collect())
    }
}
