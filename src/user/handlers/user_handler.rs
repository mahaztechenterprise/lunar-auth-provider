use std::sync::Arc;
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use super::super::super::database::configuration::mysql_db_config::PoolConnection;
use super::user_service::{AddCustomAttribute, GetCustomAttribute, GetUser };
use crate::user::handlers::user_service::CreateUser;

pub async fn create_user(
    State(data): State<Arc<PoolConnection>>,
    Json(body): Json<CreateUser>) 
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let query_result = body
        .create(data, body.clone())
        .await;

    return match query_result {
        Ok(ret) => Ok(Json(serde_json::json!(ret))),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(serde_json::json!(e))))
    };
}

pub async fn add_custom_attributes(
    State(data): State<Arc<PoolConnection>>,
    Json(body): Json<Vec<AddCustomAttribute>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let query_result = 
        AddCustomAttribute::add_custom_attributes(data, body)
        .await;

    match query_result {
        Ok(ret) => Ok(Json(serde_json::json!(ret))),
        Err(e) => Err((StatusCode::BAD_REQUEST, 
            Json(serde_json::json!(e)))),
    }
}

pub async fn get_active_user_by_id(
    State(data): State<Arc<PoolConnection>>,
    Path(id): Path<uuid::Uuid>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    let query_result = CreateUser::get_active_user(data, id.to_string()).await;

    if query_result.is_empty() {
        return Err((StatusCode::NOT_FOUND,
            Json(serde_json::json!(
                { "status": "error", "message": "No user found or is not active" }))));
    }

    let mut attributes: Vec<GetCustomAttribute> = Vec::new();
    for row in &query_result {
        if let (Some(attr_id), Some(attr_key), Some(attr_value)) = 
            (row.attr_id.clone(), row.attr_key.clone(), row.attr_value.clone()) {
                attributes.push(GetCustomAttribute {
                    id: attr_id,
                    key: attr_key,
                    value: attr_value,
                });
            }
    }

    let app_user = GetUser {
        id: query_result[0].id.clone(),
        username: query_result[0].username.clone(),
        name: query_result[0].name.clone(),
        attributes: attributes,
    };

    return Ok(Json(serde_json::json!(app_user)));
}