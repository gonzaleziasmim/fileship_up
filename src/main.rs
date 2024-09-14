mod routes;
mod services;
mod middleware;
mod error;
mod db;

use axum::{
    extract::{Extension, Multipart}, http::StatusCode, response::IntoResponse, routing::post, Router
};
use middleware::jwt_auth::AuthenticatedUser;
use std::net::SocketAddr;
use std::sync::Arc;
use dotenvy::dotenv;
use crate::db::init_db;
use crate::routes::upload_file;
use mongodb::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize the MongoDB database
    let db = init_db().await.unwrap();
    let db = Arc::new(db);

    // Build the axum router
    let app = Router::new()
        .route("/upload", post(upload_handler))
        .layer(Extension(db));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server is listening on http://{}", addr);

    // Use axum's serve method to run the app
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_handler(
    Extension(db): Extension<Arc<Database>>,
    AuthenticatedUser(user_email): AuthenticatedUser, // Use the custom extractor
    multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    // Call the `upload_file` function
    upload_file(
        Extension(Arc::clone(&db)),
        middleware::jwt_auth::AuthenticatedUser(user_email),
        multipart,
    )
    .await
}
