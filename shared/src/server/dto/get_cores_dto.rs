use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CoresDTO {
    pub id: i32,
    pub ip: String,
    pub name: String,
}