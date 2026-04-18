use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}