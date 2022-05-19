mod v1;

use axum::{routing, Router};

pub fn v1() -> Router {
    Router::new().nest(
        "/v1/admission",
        Router::new()
            .route("/mutate", routing::post(self::v1::mutate_handler))
            .route("/validate", routing::post(self::v1::validate_handler)),
    )
}
