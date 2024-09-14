use crate::error::AppError;
use axum::extract::multipart::Field;
use futures_util::io::AsyncWriteExt;
use mongodb::{bson::{self, Document}, options::GridFsBucketOptions, Database};

pub async fn save_file_to_gridfs(
    db: &Database,
    file_name: &str,
    mut field: Field<'_>,
    metadata: Option<Document>, // Use this parameter
) -> Result<String, AppError> {
    // Create a GridFS bucket with specified options
    let bucket = db.gridfs_bucket(
        GridFsBucketOptions::builder()
            .bucket_name("files".to_string())
            .chunk_size_bytes(255 * 1024)
            .build(),
    );

    // Open an upload stream with metadata
    let mut upload_stream = bucket
        .open_upload_stream(file_name)
        .await
        .map_err(|e| AppError::UploadError(e.to_string()))?;

    if let Some(metadata) = metadata {

        let metadata_bytes = bson::to_vec(&metadata)
            .map_err(|e| AppError::UploadError(e.to_string()))?;
        upload_stream
            .write(&metadata_bytes)
            .await
            .map_err(|e| AppError::UploadError(e.to_string()))?;
    }

    // Upload the file in chunks
    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|e| AppError::UploadError(e.to_string()))?
    {
        upload_stream
            .write_all(&chunk)
            .await
            .map_err(|e| AppError::UploadError(e.to_string()))?;
    }

    // Flush and close the stream
    upload_stream
        .flush()
        .await
        .map_err(|e| AppError::UploadError(e.to_string()))?;
    drop(upload_stream);

    Ok(file_name.to_string())
}
