use crate::{
    application::mappers::model::ModelMapper, domain::entities::role_by_app::RoleByApp,
    infrastructure::models::role::RoleModel,
};

impl ModelMapper<RoleModel> for RoleByApp {
    fn from_entity(entity: RoleModel) -> Self {
        RoleByApp {
            organization_id: entity.organization_id,
            application_id: entity.application_id,
            role_id: entity.role_id,
            name: entity.name,
            description: Some(entity.description),
            permissions: entity.permissions,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }

    fn to_entity(&self) -> RoleModel {
        RoleModel {
            organization_id: self.organization_id,
            application_id: self.application_id,
            role_id: self.role_id,
            name: self.name.clone(),
            description: self.description.clone().unwrap_or_default(),
            permissions: self.permissions.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
