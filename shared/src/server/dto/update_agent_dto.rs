use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateAgentDto {
    pub id: i32,
    pub ip: Option<String>,
    pub port: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
}