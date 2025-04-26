use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTask {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<Option<String>>, // Option<Option<T>> handles setting to null/None
    pub completed: Option<bool>,
}
