use std::sync::Arc;
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::{MySql, QueryBuilder};
use super::super::super::database::configuration::mysql_db_config::PoolConnection;
use super::super::super::database::model::app_user::AppUser; 
use super::create_user_dto::{AddCustomAttribute, GetCustomAttribute, GetUser };

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
    Json(body): Json<Vec<AddCustomAttribute>>,
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

pub async fn get_active_user_by_id(
    State(data): State<Arc<PoolConnection>>,
    Path(id): Path<uuid::Uuid>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    let query_result = sqlx::query_as!(AppUser, r#"
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