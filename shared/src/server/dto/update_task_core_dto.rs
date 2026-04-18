use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateTaskCoreDto {
    pub id: i32,
    pub agent_id: Option<i32>,
}