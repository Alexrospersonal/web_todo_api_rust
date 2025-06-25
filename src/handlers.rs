use axum::{extract::Path, Json};

use crate::models::Todo;

pub async fn retrieve_todos() -> &'static str {
    "All todos"
}

pub async fn retrieve_todo(Path(todo_id): Path<u32>) -> String {
    format!("You get todo with id:{todo_id}")
}

pub async fn create_todo(Json(todo): Json<Todo>) -> String {
    format!("You create todo with header: {} and body: {}", todo.header, todo.body)
}

pub async fn update_todo(Path(todo_id): Path<u32>, Json(todo): Json<Todo>) -> String {
    format!("Yoou update todo id: {todo_id} header: {}, body: {}", todo.header, todo.body)
}

pub async fn delete_todo(Path(todo_id): Path<u32>) -> String {
    format!("You deleted todo with id:{todo_id}")
}

