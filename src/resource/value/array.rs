use super::ValueMirror;
use surrealdb::sql::Array;

// res.into_iter().map(|e| e.into()).collect()
pub trait IntoArray {
    fn into_array(self) -> Array;
}

impl IntoArray for Vec<ValueMirror> {
    fn into_array(self) -> Array {
        Array(self.into_iter().map(|e| e.into()).collect())
    }
}

impl IntoArray for Vec<String> {
    fn into_array(self) -> Array {
        Array(self.into_iter().map(|e| e.into()).collect())
    }
}
