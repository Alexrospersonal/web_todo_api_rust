use axum::{
    Router,
    routing::{delete, get, post, put},
};
use handlers::{create_todo, delete_todo, retrieve_todo, retrieve_todos, update_todo};

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/todos", get(retrieve_todos))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", get(retrieve_todo))
        .route("/todos/{id}", put(update_todo))
        .route("/todos/{id}", delete(delete_todo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
