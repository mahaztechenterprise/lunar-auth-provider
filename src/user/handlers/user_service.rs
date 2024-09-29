
use serde::{Deserialize, Serialize};
use sqlx::{MySql, QueryBuilder};
use std::sync::Arc;

use crate::database::model::app_user::AppUser;

use super::super::super::database::configuration::mysql_db_config::PoolConnection;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct GetUserWithPassword {
    pub id: String,
    pub username: String,
    pub password: String,
    pub is_active: i8,
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

#[derive(Debug, Serialize)]
pub struct UserResponseData {
    name: String,
    username: String,
    id: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
    status: u16,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    message: String,
}

impl CreateUser {
    pub async fn create(&self, data: Arc<PoolConnection>, arg: CreateUser) 
    -> Result<UserResponseData, ErrorResponse>
    {
        let user_id = uuid::Uuid::new_v4().to_string();
        let query_result = sqlx::query(r#"
            INSERT INTO APP_USER (id, username, password, name)
            VALUES (?, ?, ?, ?)
        "#)
        .bind(&user_id)
        .bind(&arg.username)
        .bind(bcrypt::hash(&arg.password, 12).unwrap())
        .bind(&arg.name)
        .execute(&data.db)
        .await;

        return match query_result {
            Ok(_) => Ok(UserResponseData {
                id: String::from(user_id),
                username: self.username.clone(),
                name: self.name.clone(),
            }),
            Err(_) => Err(ErrorResponse {
                message: String::from("User not created"),
                status: 400,
            }),
        };
    }

    pub async fn get_active_user(data: Arc<PoolConnection>, id: String) -> Vec<AppUser> {
        return sqlx::query_as!(AppUser, r#"
            SELECT app.id as id, username, name, 
            attributes.id as attr_id, _key as attr_key, _value as attr_value   
            FROM APP_USER app  
            left join APP_USER_CUSTOM_ATTRIBUTE attributes 
            ON (app.id = attributes.user_id) 
            where is_active = true and app.id = ? "#, 
            id.to_string())
        .fetch_all(&data.db)
        .await
        .unwrap_or_default();
    }
}

impl AddCustomAttribute  {
    
    pub async fn add_custom_attributes(
        data: Arc<PoolConnection>,
        body: Vec<AddCustomAttribute>,
    ) -> Result<SuccessResponse, ErrorResponse> {
        
        const BIND_LIMIT: usize = 65535;
    
        let mut query_builder: QueryBuilder<MySql> = 
            QueryBuilder::new("INSERT INTO APP_USER_CUSTOM_ATTRIBUTE (id, _key, _value, user_id) ");
        
        let query_result = query_builder
            .push_values(body.into_iter().take(BIND_LIMIT / 4),
            | mut b, attributes | {
                log::info!("Data, {:?}", attributes);
                b.push_bind(uuid::Uuid::new_v4().to_string());
                b.push_bind(attributes.key);
                b.push_bind(attributes.value);
                b.push_bind(attributes.user_id);
            })
            .build()
            .execute(&data.db)
            .await;
    
        match query_result {
            Ok(_) => Ok(SuccessResponse { message: String::from("custom attributes added") }),
            Err(e) => Err(ErrorResponse { 
                 status: 400, 
                 message: e.to_string()
            }),
        }
    }
}


impl GetUserWithPassword {
    pub async fn get_user_with_password(username: String, data: Arc<PoolConnection>) -> Result<Option<GetUserWithPassword>, sqlx::Error> {
        let result = sqlx::query_as!(GetUserWithPassword,
            "SELECT id, username, password, is_active FROM app_user WHERE username = ?",
            username)
            .fetch_optional(&data.db)
            .await;    
        return result;
    }
}