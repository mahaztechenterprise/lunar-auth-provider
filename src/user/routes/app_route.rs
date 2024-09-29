use axum::routing::{Router, post, get};
use std::sync::Arc;

use crate::database::configuration::mysql_db_config::PoolConnection;
use super::super::handlers::user_handler::{
    create_user, 
    add_custom_attributes,
    get_active_user_by_id,
};
use super::super::handlers::auth_handler::{login_user, validate_token};

pub fn create_routes(app_state: Arc<PoolConnection>) -> Router {
    return Router::new()
        .route("/user", post(create_user))
        .route("/user/attributes", post(add_custom_attributes))
        .route("/user/:user_id", get(get_active_user_by_id))
        .route("/login", post(login_user))
        .route("/verify", get(validate_token))
        .with_state(app_state);
}