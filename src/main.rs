use std::{error::Error, str::FromStr, sync::Arc};

use axum::{
    routing::{delete, get, post, put}, Extension, Router
};
use handlers::{create_todo, delete_todo, retrieve_todo, retrieve_todos, update_todo};
use services::Database;

mod handlers;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let state = Arc::new(Database::new().await?);

    let app = Router::new()
        .route("/todos", get(retrieve_todos))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", get(retrieve_todo))
        .route("/todos/{id}", put(update_todo))
        .route("/todos/{id}", delete(delete_todo))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
