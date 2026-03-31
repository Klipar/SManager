use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateCoreDto {
    pub ip: String,
    pub port: i32,
    pub name: String,
}