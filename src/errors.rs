use std::fmt;
use std::error::Error;
use actix_web::App;
use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DieselError),
    ValidationError(String),
    NotFoundError(String),
    UnknownError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "NotFound error: {}", msg),
            AppError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl Error for AppError {}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        AppError::DatabaseError(err)
    }
}