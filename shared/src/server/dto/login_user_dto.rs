use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUserDto {
    pub login: String,
    pub password: String,
}