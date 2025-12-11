use chrono::{DateTime, Utc};

pub struct Todo {
    pub id: i32,
    pub title: String,
    pub created: DateTime<Utc>,
    pub completed: Option<DateTime<Utc>>,
}
