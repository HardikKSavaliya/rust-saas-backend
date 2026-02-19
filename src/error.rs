use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::error;

/// Application error type
///
/// Uses `thiserror` for structured error types that can be returned from handlers.
/// Uses `anyhow` internally for error chaining and context in Internal errors.
///
/// # Usage
///
/// ```rust
/// // Return error directly (AppError implements IntoResponse)
/// pub async fn handler() -> AppError {
///     AppError::NotFound("User not found".to_string())
/// }
///
/// // Use Result with AppError
/// pub async fn handler() -> AppResult<String> {
///     let result: Result<String, String> = some_operation()?;
///     Ok(result)
/// }
///
/// // Convert anyhow::Error to AppError (automatic via From trait)
/// pub async fn handler() -> AppResult<String> {
///     let value: String = some_anyhow_operation().map_err(AppError::from)?;
///     Ok(value)
/// }
/// ```
#[derive(Error, Debug)]
#[allow(dead_code)] // Error variants are part of public API and will be used as features are added
pub enum AppError {
    /// 400 Bad Request
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 401 Unauthorized
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 403 Forbidden
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 404 Not Found
    #[error("Not found: {0}")]
    NotFound(String),

    /// 409 Conflict
    #[error("Conflict: {0}")]
    Conflict(String),

    /// 422 Unprocessable Entity
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// 500 Internal Server Error
    /// Can wrap anyhow::Error to preserve error chains
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    /// Database errors
    #[error("Database error: {0}")]
    Database(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[source] config::ConfigError),

    /// Serialization/Deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[source] serde_json::Error),
}

/// Error response structure
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl AppError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Serialization(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Get the error code string
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::Serialization(_) => "SERIALIZATION_ERROR",
        }
    }

    /// Log the error appropriately
    /// Errors are always logged, regardless of environment
    pub fn log_error(&self) {
        let status = self.status_code();
        if status.is_server_error() {
            // Always log server errors (ERROR level)
            error!("Server error: {}", self);
        } else if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
            // Auth errors - log as ERROR in production, WARN in development
            error!("Auth error: {}", self);
        } else {
            // Client errors - log as ERROR in production, DEBUG in development
            // But since we want errors to show in production, log as ERROR
            error!("Client error: {}", self);
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the error
        self.log_error();

        let status = self.status_code();
        let error_response = ErrorResponse {
            error: self.error_code().to_string(),
            message: self.to_string(),
            details: None,
        };

        (status, Json(error_response)).into_response()
    }
}

// Conversion implementations for common error types
// Note: serde_json::Error and config::ConfigError are handled via #[source] attribute above
// anyhow::Error is handled via #[from] attribute above

// Database error conversions (for future SeaORM integration)
// Uncomment when SeaORM is added:
// impl From<sea_orm::DbErr> for AppError {
//     fn from(err: sea_orm::DbErr) -> Self {
//         match err {
//             sea_orm::DbErr::RecordNotFound(_) => AppError::NotFound("Resource not found".to_string()),
//             sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sqlx_err)) => {
//                 if let Some(db_err) = sqlx_err.as_database_error() {
//                     if db_err.code().as_deref() == Some("23505") {
//                         AppError::Conflict("Duplicate entry".to_string())
//                     } else {
//                         AppError::Database(db_err.to_string())
//                     }
//                 } else {
//                     AppError::Database(err.to_string())
//                 }
//             }
//             _ => AppError::Database(err.to_string()),
//         }
//     }
// }

/// Result type alias for convenience
pub type AppResult<T> = Result<T, AppError>;

/// Extension trait for Result to add context
/// Part of public API for library users
#[allow(dead_code)]
pub trait ResultExt<T> {
    fn context(self, msg: &str) -> AppResult<T>;
    fn with_context<F>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    /// Add context to an error, converting it to AppError
    /// Uses anyhow for error chaining
    fn context(self, msg: &str) -> AppResult<T> {
        self.map_err(|e| {
            let anyhow_err = anyhow::Error::from(e).context(msg.to_string());
            AppError::Internal(anyhow_err)
        })
    }

    /// Add context with a closure, converting it to AppError
    /// Uses anyhow for error chaining
    fn with_context<F>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let anyhow_err = anyhow::Error::from(e).context(f());
            AppError::Internal(anyhow_err)
        })
    }
}

/// Helper to convert anyhow::Error to AppError with context
impl AppError {
    /// Create an internal error from anyhow with additional context
    /// Part of public API for library users
    #[allow(dead_code)]
    pub fn internal_with_context(err: anyhow::Error, context: String) -> Self {
        AppError::Internal(err.context(context))
    }

    /// Create an internal error from a string message
    pub fn internal(msg: impl Into<String>) -> Self {
        AppError::Internal(anyhow::anyhow!(msg.into()))
    }
}
