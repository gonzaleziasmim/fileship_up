use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("MongoDB error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("File upload error: {0}")]
    UploadError(String),

    #[error("Invalid token")]
    InvalidToken,
}
