use axum::{extract::Multipart, Extension, response::IntoResponse, http::StatusCode};
use mongodb::bson::Uuid;
use mongodb::{bson::doc, Database};
use crate::services::file_service::save_file_to_gridfs;
use crate::middleware::jwt_auth::AuthenticatedUser;
use std::sync::Arc;

pub async fn upload_file(
    Extension(db): Extension<Arc<Database>>,
    AuthenticatedUser(user_email): AuthenticatedUser,
    mut multipart: Multipart
) -> Result<impl IntoResponse, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let uid = Uuid::new();
        let file_metadata = doc! { "user_email": user_email.clone(), "file_name": &file_name, "uid": uid };

        match save_file_to_gridfs(&db, &file_name, field, Some(file_metadata)).await {
            Ok(file_id) => {
                println!("User {} uploaded a file with ID: {}", user_email, file_id);
                return Ok((StatusCode::OK, format!("File uploaded with ID: {}", file_id)));
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    Err(StatusCode::BAD_REQUEST)
}
