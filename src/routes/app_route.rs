use axum::routing::Router;

use super::auth_route::login;

pub fn create_routes() -> Router {
    let router: Router = Router::new();
    let router: Router = login(router);

    return router;
}