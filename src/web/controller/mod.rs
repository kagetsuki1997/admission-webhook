mod admission;

use axum::Router;

pub fn api_v1_index() -> Router {
    Router::new().nest("/api", Router::new().merge(self::admission::v1()))
}
