use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct AppUser {
    pub id: String,
    pub name: String,
    pub username: String,
    pub attr_id: std::option::Option<String>,
    pub attr_key: std::option::Option<String>,
    pub attr_value: std::option::Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct AppRole {
    pub id: String,
    pub role_name: String,
    pub is_active: i8,
    pub user_id: String,
}