use axum::routing::{Router, post};
use std::sync::Arc;

use crate::database::configuration::mysql_db_config::PoolConnection;
use super::super::handlers::user_handler::create_user;

pub fn create_routes(app_state: Arc<PoolConnection>) -> Router {
    return Router::new()
        .route("/user", post(create_user))
        .with_state(app_state);
}