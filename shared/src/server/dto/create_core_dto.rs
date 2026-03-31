use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateCoreDto {
    pub ip: String,
    pub name: String,
}