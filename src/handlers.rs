use std::sync::Arc;

use axum::{Extension, Json, extract::Path};

use crate::{
    models::{Todo, UpdateTodo},
    services::{
        Database, create_one_todo, delete_one_todo, retrieve_todo_from_db, retrieve_todos_from_db,
        update_one_todo,
    },
};

pub async fn retrieve_todos(Extension(db): Extension<Arc<Database>>) -> Json<Vec<Todo>> {
    match retrieve_todos_from_db(&db.pool).await {
        Some(todos) => Json(todos),
        None => Json(Vec::new()),
    }
}

pub async fn retrieve_todo(
    Path(todo_id): Path<u32>,
    Extension(db): Extension<Arc<Database>>,
) -> Json<Option<Todo>> {
    Json(retrieve_todo_from_db(&db.pool, todo_id).await)
}

pub async fn create_todo(
    Extension(db): Extension<Arc<Database>>,
    Json(todo): Json<Todo>,
) -> Json<i64> {
    Json(create_one_todo(&db.pool, &todo).await)
}

pub async fn update_todo(
    Extension(db): Extension<Arc<Database>>,
    Path(todo_id): Path<i64>,
    Json(todo): Json<UpdateTodo>,
) -> Json<bool> {
    Json(update_one_todo(&db.pool, &todo, todo_id).await)
}

pub async fn delete_todo(
    Path(todo_id): Path<i64>,
    Extension(db): Extension<Arc<Database>>,
) -> Json<bool> {
    Json(delete_one_todo(&db.pool, todo_id).await)
}
