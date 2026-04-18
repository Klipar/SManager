use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetLogsDto {
    pub agent_id: Option<i32>,
    pub task_id: Option<i32>,
    pub user_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}