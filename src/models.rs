use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct Category {
    pub category_id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub slots: i8,
}
