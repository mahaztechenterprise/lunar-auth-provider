use std::sync::Arc;
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{ Serialize, Deserialize };
use sqlx::{MySql, QueryBuilder};
use super::super::super::database::configuration::mysql_db_config::PoolConnection;
use super::user_service::GetUserWithPassword;


#[derive(Debug, Serialize)]

struct AuthResponse {
    access_token: String,
    refresh_token: String,
    scope: String,
    expires_at: i32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}


fn verify_password(password: String, user: GetUserWithPassword) -> bool {
    let is_verified = bcrypt::verify(password, &user.password);
    match is_verified {
        Ok(verified) => verified,
        Err(_) => false 
    }
}

pub async fn login_user(
    State(data): State<Arc<PoolConnection>>,
    Json(body): Json<AuthRequest>) 
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let username = body.username;

    let result = sqlx::query_as!(GetUserWithPassword,
        "SELECT id, password, is_active FROM app_user WHERE username = ?",
        username)
        .fetch_optional(&data.db)
        .await;

    if result.is_err() {
        return Err((StatusCode::CONFLICT, 
            Json(serde_json::json!({"status": "error", "message": "Internal Error"}))))
    }

    let res = match result {
        Ok(res) => res,
        _ => None
    };

    if res.is_none() {
        return Err((StatusCode::UNAUTHORIZED, 
            Json(serde_json::json!({"status": "error", "message": "username/password is not valid"}))))
    }

    let error_message = serde_json::json!(
        {"status": "error", "message": "username/password not verified"}
    );

    let user = res.unwrap();
    if user.is_active == 0 {
        return Err((StatusCode::UNAUTHORIZED, 
            Json((error_message))))
    }
    
    let is_verified = verify_password(body.password, user);
    
    if is_verified {
        return Err((StatusCode::UNAUTHORIZED, 
            Json((error_message))));
    }

    return Ok(Json(serde_json::json!({ "status": "success" })));
}