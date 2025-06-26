use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub HEADER: String,
    pub BODY: String,
}
