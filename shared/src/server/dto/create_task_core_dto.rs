use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskCoreDto {
    pub id: i32,
    pub agent_id: i32,
}