use axum::routing::Router;
use axum::routing::post;

pub fn login(routes: Router) -> Router {

    async fn handler() ->  &'static str {
        return "Logged In";
    }

    return routes.route("/oauth/login", post(handler));
}