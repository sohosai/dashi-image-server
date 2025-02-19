use axum::{http::StatusCode, response::IntoResponse};
use domain::value_object::error::AppError;

#[utoipa::path(
    get,
    path = "/",
    tag = "Ping",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn ping_handler() -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached ping handler.");
    Ok((StatusCode::OK, "pong".to_string()).into_response())
}
