use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::{StatusCode, header::ToStrError},
    response::IntoResponse,
};
use bcrypt::BcryptError;
use sea_orm::DbErr;
use serde::Serialize;
use validator::ValidationErrors;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    status_code: StatusCode,
    message: String,
    validation_errors: Option<ValidationErrors>,
}

impl AppError {
    pub fn system_error(message: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("System error: {}", message),
            validation_errors: None,
        }
    }

    pub fn illegal_media_type(message: String) -> Self {
        Self {
            status_code: StatusCode::UNSUPPORTED_MEDIA_TYPE,
            message,
            validation_errors: None,
        }
    }

    pub fn illegal_arguments<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: message.to_string(),
            validation_errors: None,
        }
    }

    pub fn not_found<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self {
            status_code: StatusCode::NOT_FOUND,
            message: message.to_string(),
            validation_errors: None,
        }
    }

    pub fn access_denied<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self {
            status_code: StatusCode::FORBIDDEN,
            message: message.to_string(),
            validation_errors: None,
        }
    }

    pub fn unauthorized<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self {
            status_code: StatusCode::UNAUTHORIZED,
            message: message.to_string(),
            validation_errors: None,
        }
    }
}

impl From<JsonRejection> for AppError {
    fn from(error: JsonRejection) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: format!("Wrong json: {}", error),
            validation_errors: None,
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        Self {
            status_code: StatusCode::UNPROCESSABLE_ENTITY,
            message: error.to_string(),
            validation_errors: Some(error),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::system_error(format!("IO error: {}", error))
    }
}

impl From<DbErr> for AppError {
    fn from(error: DbErr) -> Self {
        AppError::system_error(format!("DbErr: {}", error))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::system_error(format!("Sqlx error: {}", error))
    }
}

impl From<ToStrError> for AppError {
    fn from(error: ToStrError) -> Self {
        AppError::system_error(format!("ToStrError: {}", error))
    }
}

impl From<BcryptError> for AppError {
    fn from(error: BcryptError) -> Self {
        AppError::system_error(format!("BcryptError: {}", error))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        AppError::system_error(format!("JWTError: {}", error))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(AppErrorResponse::new(self))).into_response()
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_errors: Option<ValidationErrors>,
}

impl AppErrorResponse {
    pub fn new(error: AppError) -> Self {
        Self {
            error: error.message,
            validation_errors: error.validation_errors,
        }
    }
}
