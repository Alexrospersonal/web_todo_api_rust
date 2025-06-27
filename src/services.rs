use std::str::FromStr;

use sqlx::{
    Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::models::Todo;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_options = SqliteConnectOptions::from_str("sqlite:data.db")?
            .create_if_missing(true)
            .to_owned();

        let pool = SqlitePoolOptions::new().connect_with(db_options).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS TODOS (id INTEGER PRIMARY KEY,HEADER TEXT NOT NULL,BODY TEXT);",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}

// TODO: add limit with option in parameters
pub async fn retrieve_todos_from_db(pool: &SqlitePool) -> Option<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM TODOS")
        .fetch_all(pool)
        .await
        .unwrap();
    // TODO: add error validation
    if todos.is_empty() { None } else { Some(todos) }
}

pub async fn retrieve_todo_from_db_as_json(pool: &SqlitePool, id: u32) -> Option<Todo> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM TODOS WHERE id=?")
        .bind(id)
        .fetch_one(pool)
        .await;

    match todo {
        Ok(todo) => Some(todo),
        // TODO: add error validation
        Err(_) => None,
    }
}


