use crate::{
    application::{
        dto::response::user::CreateUserResponse, mappers::application::ApplicationMapper,
    },
    infrastructure::models::user::FoundUserModel,
};

pub struct FoundUserMapper;
impl ApplicationMapper<FoundUserModel, CreateUserResponse> for FoundUserMapper {
    fn to_dto(model: FoundUserModel) -> CreateUserResponse {
        CreateUserResponse {
            user_id: model.user_id.to_string(),
            username: model.username,
            email: model.email,
        }
    }
}
