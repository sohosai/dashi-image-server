use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    SetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error(transparent)]
    SetupAxumError(#[from] std::io::Error),
    #[error(transparent)]
    SetupCORSError(#[from] axum::http::header::InvalidHeaderValue),
}
