use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AppUser {
    pub id: String,
    pub name: String,
    pub username: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AppUserCustomAttribute {
    pub id: String,
    pub user_id: String,
    pub key: String,
    pub value: String,
}