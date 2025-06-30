use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub HEADER: String,
    pub BODY: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub HEADER: Option<String>,
    pub BODY: Option<String>
}