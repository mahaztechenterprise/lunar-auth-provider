use std::{borrow::BorrowMut, sync::Arc};
use axum::{
    extract::{ Path, Query, State },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use super::{super::super::database::configuration::mysql_db_config::PoolConnection, create_user_dto::CreateUser}; 

#[derive(Debug, Serialize)]
struct UserResponseData {
    name: String,
    username: String,
    id: String,
}

pub async fn create_user(
    State(data): State<Arc<PoolConnection>>,
    Json(body): Json<super::create_user_dto::CreateUser>) 
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let user_id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"
        INSERT INTO APP_USER (id, username, password, name)
        VALUES (?, ?, ?, ?)
    "#)
    .bind(&user_id)
    .bind(&body.username)
    .bind(bcrypt::hash(&body.password, 12).unwrap())
    .bind(&body.name)
    .execute(&data.db)
    .await;

    if let Err(err) = query_result {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("{:?}", err)
        });

        return Err((StatusCode::CONFLICT, Json(error_response)))
    };

    let user_data = UserResponseData {
        name: body.name.to_owned(),
        username: body.username.to_owned(),
        id: String::from(user_id),
    };

    let response = serde_json::json!(
        {
            "status": "SUCCESS",
            "data": user_data,
        });
    return Ok(Json(response));
}

pub async fn get_users() -> &'static str {
    return "List of Users";
}

pub async fn is_valid_user() -> &'static str {
    return "The user is valid";
}