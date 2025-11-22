use axum::extract::Multipart;
use crate::services::watermark_service;

pub async fn process_watermark(multipart: Multipart) -> impl axum::response::IntoResponse {
    let zip_bytes = watermark_service::process(multipart).await;

    (
        axum::http::StatusCode::OK,
        [
            ("Content-Type", "application/zip"),
            ("Content-Disposition", "attachment; filename=\"hasil_watermark.zip\"")
        ],
        zip_bytes
    )
}
