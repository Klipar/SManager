use serde::Serialize;

use super::user_response_dto::UserResponseDto;

#[derive(Serialize)]
pub struct LoginResponseDto {
    pub token: String,
    pub user: UserResponseDto,
}