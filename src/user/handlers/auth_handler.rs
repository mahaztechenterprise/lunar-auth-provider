use std::sync::Arc;
use axum::http::HeaderMap;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{ Serialize, Deserialize };
use super::super::super::database::configuration::mysql_db_config::PoolConnection;
use super::user_service::GetUserWithPassword;


#[derive(Debug, Serialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    scope: String,
    expires_at: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaim {
    sub: String,
    iat: String,
    exp: u32,
    id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}


fn verify_password(password: String, user: &GetUserWithPassword) -> bool {
    let is_verified = bcrypt::verify(password, &user.password);
    match is_verified {
        Ok(verified) => verified,
        Err(_) => false 
    }
}

fn create_token(user: &GetUserWithPassword) -> Result<String, jsonwebtoken::errors::Error> {
    let token_claim = TokenClaim {
        iat: String::from("iat"),
        id: user.id.clone(),
        exp: 10000,
        sub: user.username.clone(),
    };

    let jwt_secret = std::env::var("JWS_SECRET").unwrap();

    let token = encode(
        &Header::default(), 
        &token_claim, 
        &EncodingKey::from_secret(jwt_secret.as_ref()));

    return token;
}

pub async fn login_user(
    State(data): State<Arc<PoolConnection>>,
    Json(body): Json<AuthRequest>) 
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let username = body.username;

    let result = 
        GetUserWithPassword::get_user_with_password(username, data).await;

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
            Json(error_message)))
    }
    
    let is_verified = verify_password(body.password, &user);
    
    if !is_verified {
        return Err((StatusCode::UNAUTHORIZED, 
            Json(error_message)));
    }

    let auth_response = match create_token(&user) {
        Ok(token) => Json(serde_json::json!(AuthResponse {
            access_token: token,
            expires_at: 10000,
            refresh_token: String::from("not implemented"),
            scope: String::from("not implemented"),
        })),
        Err(_err) => Json(serde_json::json!({ "status": "failed", "message": "failed to create token"})),
    };

    return Ok(auth_response);
}

pub fn refresh_the_token() {
    todo!("implement handler for refreshing token")
}

pub async fn validate_token(headers: HeaderMap) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let bearer = String::from("Bearer ");
    let authorize = headers.get("Authorization");
    
    let get_token = authorize.ok_or(
        Err::<(), (StatusCode, Json<serde_json::Value>)>(
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"status": "failed"})))
        )
    )
    .unwrap()
    .to_str()
    .map(|op| op.get(bearer.len()..))  // Note the fixed slice: starts from bearer length
    .map_err(|_err| Err::<(), (StatusCode, Json<serde_json::Value>)>(
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"status": "failed"})))
    ));
    
    if get_token.as_ref().is_err() || get_token.as_ref().unwrap().is_none() {
        return Err(
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "status": "failed" })))
        );
    }

    let token = get_token.unwrap().unwrap();

    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let token = 
        jsonwebtoken::decode::<TokenClaim>(token, 
                &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::default());
    
    match token {
        Ok(TokenData { header: _, claims } ) => 
            Ok(Json(serde_json::json!(claims))),
        Err(_) => 
            Err(
                (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "status": "failed" })))
            ),
    }
}
