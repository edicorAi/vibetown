use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account is disabled")]
    AccountDisabled,

    #[error("Session expired")]
    SessionExpired,

    #[error("Session revoked")]
    SessionRevoked,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Provider not configured")]
    ProviderNotConfigured,

    #[error("Operation not supported by this provider")]
    NotSupported,

    #[error("OIDC error: {0}")]
    Oidc(String),

    #[error("LDAP error: {0}")]
    Ldap(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::AccountDisabled => StatusCode::FORBIDDEN,
            AuthError::SessionExpired | AuthError::SessionRevoked | AuthError::Unauthorized => {
                StatusCode::UNAUTHORIZED
            }
            AuthError::Forbidden => StatusCode::FORBIDDEN,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthError::ProviderNotConfigured | AuthError::NotSupported => {
                StatusCode::BAD_REQUEST
            }
            AuthError::Oidc(_) | AuthError::Ldap(_) => StatusCode::BAD_GATEWAY,
            AuthError::Database(_) | AuthError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = serde_json::json!({ "error": self.to_string() });
        (status, axum::Json(body)).into_response()
    }
}
