
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub password: String,
}
