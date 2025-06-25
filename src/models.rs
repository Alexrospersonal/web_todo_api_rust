use serde::Deserialize;

#[derive(Deserialize)]
pub struct Todo {
    pub header: String,
    pub body: String,
}
