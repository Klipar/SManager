use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateCoreDto {
    pub ip: String,
    pub name: String,
    pub client_cn: String
}