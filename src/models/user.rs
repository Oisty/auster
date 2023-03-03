use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: Option<i64>,
    product_type: Option<String>,
    name: Option<String>,
}
