use axum::{extract::rejection::JsonRejection, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

use sqlx::Error as SqlxError;
#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    JsonExtractionRejection(#[from] JsonRejection),
    #[error("{0}")]
    BadRequest(String),
    #[error("")]
    NotFound(String),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Failed to send Email")]
    EmailError,
    #[error("{0}")]
    TransactionError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::JsonExtractionRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
            ApiError::BadRequest(message) => (axum::http::StatusCode::BAD_REQUEST, message),
            ApiError::NotFound(message) => (axum::http::StatusCode::NOT_FOUND, message),
            ApiError::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            ApiError::EmailError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send Email".to_string(),
            ),
            ApiError::TransactionError(message) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                message.to_string(),
            ),
        };

        let payload = json!({
            "message": message,
            "status": status.as_u16(),
        });
        tracing::error!("Error: {}", message);
        (status, Json(payload)).into_response()
    }
}

// FriendError is enum type which is used to handle error
pub enum FriendError {
    FriendNotFound, // FriendNotFound is used when friend is not found in the database table
    // Ex: When we try to remove or get friend which is not in the list
    FriendAlreadyExist, // FriendAlreadyExist is used when friend with provided email already exist in the database table
    SqlxError(SqlxError), // SqlxError is used when sqlx crate return error
}

pub enum UserError {
    UserNotFound,
    UserAlreadyExist,
    SqlxError(SqlxError),
}
