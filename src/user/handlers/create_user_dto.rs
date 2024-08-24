
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GetUser {
    pub id: String,
    pub name: String,
    pub username: String,
    pub attributes: Vec<GetCustomAttribute>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddCustomAttribute {
    pub id: String,
    pub key: String,
    pub value: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GetCustomAttribute {
    pub id: String,
    pub key: String,
    pub value: String,
}
