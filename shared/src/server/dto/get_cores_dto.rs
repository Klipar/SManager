use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetAllCoresDTO {
    pub id: i32,
    pub ip: String,
    pub port: i32,
    pub name: String,
}