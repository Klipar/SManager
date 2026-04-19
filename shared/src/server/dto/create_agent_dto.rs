use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAgentDto {
    pub ip: String,
    pub port: i32,
    pub name: String,
    pub description: Option<String>,
}