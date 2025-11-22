use axum::{Router, routing::post};

mod watermark;

pub fn routes() -> Router {
    Router::new()
        .route("/watermark", post(watermark::process_watermark))
}
