use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskCore {
    pub id: i32,
    pub agent_id: i32,
}