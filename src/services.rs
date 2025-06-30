use std::str::FromStr;

use sqlx::{
    QueryBuilder, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteQueryResult},
};

use crate::models::{Todo, UpdateTodo};

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

pub async fn retrieve_todo_from_db(pool: &SqlitePool, id: u32) -> Option<Todo> {
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

pub async fn create_one_todo(pool: &SqlitePool, todo: &Todo) -> i64 {
    let row = sqlx::query!("SELECT COUNT(*) as count FROM TODOS;")
        .fetch_one(pool)
        .await
        .unwrap();
    let next_id = row.count + 1;

    let result: SqliteQueryResult =
        sqlx::query("INSERT INTO TODOS (ID, HEADER, BODY) VALUES (?, ?, ?);")
            .bind(next_id)
            .bind(&todo.HEADER)
            .bind(&todo.BODY)
            .execute(pool)
            .await
            .unwrap();

    if result.rows_affected() > 0 {
        next_id
    } else {
        -1
    }
}

pub async fn delete_one_todo(pool: &SqlitePool, id: i64) -> bool {
    let delete_result = sqlx::query("DELETE FROM TODOS WHERE ID = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();

    if delete_result.rows_affected() > 0 {
        true
    } else {
        false
    }
}

pub async fn update_one_todo(pool: &SqlitePool, todo: &UpdateTodo, id: i64) -> bool {
    let mut query_builder: QueryBuilder<'_, Sqlite> = QueryBuilder::new("UPDATE TODOS SET ");

    if todo.HEADER.is_none() && todo.BODY.is_none() {
        return false;
    }

    if let Some(header) = &todo.HEADER {
        query_builder
            .push("HEADER = ")
            .push_bind(header)
            .push(", ");
    }

    if let Some(body) = &todo.BODY {
        query_builder.push("BODY = ").push_bind(body).push(" ");
    }

    query_builder.push("WHERE ID = ").push_bind(id).push(" ;");
    let query = query_builder.build();

    let updated_result = query.execute(pool).await.unwrap();

    if updated_result.rows_affected() > 0 {
        true
    } else {
        false
    }
}
