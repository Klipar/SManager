use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Core {
    pub id: i32,
    pub ip: String,
    pub port: i32,
    pub name: String,
    pub token_hash: String,
}