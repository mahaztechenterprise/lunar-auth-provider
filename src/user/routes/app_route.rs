use axum::routing::{Router, post};
use std::sync::Arc;

use crate::database::configuration::mysql_db_config::PoolConnection;
use super::super::handlers::user_handler::{create_user, add_custom_attributes};

pub fn create_routes(app_state: Arc<PoolConnection>) -> Router {
    return Router::new()
        .route("/user", post(create_user))
        .route("/user/attributes", post(add_custom_attributes))
        .with_state(app_state);
}