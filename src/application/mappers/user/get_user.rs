use crate::{
    application::{dto::response::user::GetUserResponse, mappers::application::ApplicationMapper},
    infrastructure::models::user::CleannedUserModel,
};

pub struct GetUserMapper;
impl ApplicationMapper<CleannedUserModel, GetUserResponse> for GetUserMapper {
    fn to_dto(model: CleannedUserModel) -> GetUserResponse {
        GetUserResponse {
            user_id: model.user_id,
            organization_id: model.organization_id,
            application_id: model.application_id,
            username: model.username,
            email: model.email,
            created_at: model.created_at,
            updated_at: model.updated_at,
            is_active: model.is_active,
            is_verified: model.is_verified,
            is_locked: model.is_locked,
            last_login: model.last_login,
            mfa_enabled: model.mfa_enabled,
            deactivated_at: model.deactivated_at,
        }
    }
}
