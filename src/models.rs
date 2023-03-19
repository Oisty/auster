pub struct Category {
    pub id: String,
    pub name: String,
}

pub struct Room {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub slots: i8,
}
