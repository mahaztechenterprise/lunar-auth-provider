use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::{MySql, QueryBuilder};
use super::super::super::database::configuration::mysql_db_config::PoolConnection; 
use super::create_user_dto::AddCustomAttributes;

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

pub async fn add_custom_attributes(
    State(data): State<Arc<PoolConnection>>,
    Json(body): Json<Vec<AddCustomAttributes>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
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
        Ok(_) => Ok(Json(serde_json::json!({ "status": "SUCCESS" }))),
        Err(e) => Err((StatusCode::CONFLICT, 
            Json(serde_json::json!({ "status": "error", "message": e.to_string() })))),
    }
}