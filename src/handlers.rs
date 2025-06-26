use std::sync::Arc;

use axum::{extract::Path, Extension, Json};

use crate::{models::Todo, services::{retrieve_todos_from_db_as_json, Database}};

pub async fn retrieve_todos(Extension(db): Extension<Arc<Database>>) -> Json<Vec<Todo>>{
    match retrieve_todos_from_db_as_json(&db.pool).await {
        Some(todos) => Json(todos),
        None => Json(Vec::new()),
    }
}

pub async fn retrieve_todo(Path(todo_id): Path<u32>) -> String {
    format!("You get todo with id:{todo_id}")
}

pub async fn create_todo(Json(todo): Json<Todo>) -> String {
    format!("You create todo with header: {} and body: {}", todo.HEADER, todo.BODY)
}

pub async fn update_todo(Path(todo_id): Path<u32>, Json(todo): Json<Todo>) -> String {
    format!("Yoou update todo id: {todo_id} header: {}, body: {}", todo.HEADER, todo.BODY)
}

pub async fn delete_todo(Path(todo_id): Path<u32>) -> String {
    format!("You deleted todo with id:{todo_id}")
}
