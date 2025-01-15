use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleResponse {
    pub value: String,
}

impl SimpleResponse {
    pub fn new(value: String) -> SimpleResponse {
        SimpleResponse { value }
    }
}

#[derive(Serialize, Debug)]
pub struct DataSource {
    pub id: sqlx::types::Uuid,
    pub name: String,
}
