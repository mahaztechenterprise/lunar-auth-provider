
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AddCustomAttributes {
    pub user_id: String,
    pub key: String,
    pub value: String,
}