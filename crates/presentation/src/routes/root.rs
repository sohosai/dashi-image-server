use axum::routing::{get, put};
use axum::Router;

pub fn util_route() -> Router<RwLockSharedState> {
    Router::new().route("/healthcheck", get(healthcheck_handler))
}

pub fn image_route() -> Router<RwLockSharedState> {
    Router::new().route("/image", put(update_handler))
}

pub fn root_route() -> Router<RwLockSharedState> {
    let root_routes = Router::new().merge(util_route()).merge(image_route());
    Router::new().nest("/api", root_routes)
}
