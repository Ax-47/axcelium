#[cfg(test)]
mod tests {
    use crate::application::dto::payload::user::CreateUserPayload;
    use crate::application::dto::response::user::CreateUserResponse;
    use crate::application::repositories::users::create::MockCreateUserRepository;
    use crate::application::services::users::create::{CreateUserService, CreateUserServiceImpl};
    use crate::domain::entities::apporg_client_id::CleanAppOrgByClientId;
    use crate::domain::entities::user::User;
    use crate::domain::entities::user_organization::UserOrganization;
    use crate::domain::value_objects::app_config::AppConfig;
    use chrono::Utc;
    use scylla::value::CqlTimestamp;
    use std::sync::Arc;
    use uuid::Uuid;
    trait Fake {
        fn fake() -> Self;
        fn to_response(&self) -> CreateUserResponse;
    }
    #[cfg(test)]
    impl Fake for User {
        fn fake() -> Self {
            User::new(
                Uuid::new_v4(), // application_id
                Uuid::new_v4(), // organization_id
                "fake_user".to_string(),
                "hashed_password".to_string(),
                Some("fake@example.com".to_string()),
            )
        }

        fn to_response(&self) -> CreateUserResponse {
            return CreateUserResponse {
                user_id: self.user_id.to_string(),
                username: self.username.clone(),
                email: self.email.clone(),
            };
        }
    }
    fn build_apporg(can_email_null: bool, must_name_unique: bool) -> CleanAppOrgByClientId {
        let now = CqlTimestamp(Utc::now().timestamp_millis());
        CleanAppOrgByClientId {
            client_id: Uuid::new_v4(),
            application_id: Uuid::new_v4(),
            organization_id: Uuid::new_v4(),
            organization_name: "Test Org".into(),
            organization_slug: "test-org".into(),
            application_name: "App".into(),
            application_description: "desc".into(),
            application_config: AppConfig {
                can_allow_email_nullable: can_email_null,
                is_must_name_unique: must_name_unique,
            }
            .to_string(),
            contact_email: "admin@test.org".into(),
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    fn build_payload(email: Option<String>, username: &str) -> CreateUserPayload {
        CreateUserPayload {
            email,
            username: username.into(),
            password: "secret".into(),
        }
    }

    #[tokio::test]
    async fn test_username_too_short() {
        let mock = MockCreateUserRepository::new();
        let service = CreateUserServiceImpl::new(Arc::new(mock));

        let payload = build_payload(Some("mail@example.com".into()), "a");
        let apporg = build_apporg(true, false);

        let res = service.execute(apporg, payload).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, 400);
    }

    #[tokio::test]
    async fn test_email_required_but_missing() {
        let mock = MockCreateUserRepository::new();
        let service = CreateUserServiceImpl::new(Arc::new(mock));

        let payload = build_payload(None, "mikachan");
        let apporg = build_apporg(false, false); // can't be null

        let res = service.execute(apporg, payload).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, 400);
    }

    #[tokio::test]
    async fn test_email_already_used() {
        let mut mock = MockCreateUserRepository::new();
        mock.expect_find_user_by_email()
            .returning(|_, _, _| Ok(Some(User::fake().to_response()))); // <- สมมุติว่ามี method fake()

        let service = CreateUserServiceImpl::new(Arc::new(mock));

        let payload = build_payload(Some("taken@example.com".into()), "mikachan");
        let apporg = build_apporg(false, false);

        let res = service.execute(apporg, payload).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, 400);
    }

    #[tokio::test]
    async fn test_username_already_used() {
        let mut mock = MockCreateUserRepository::new();
        mock.expect_find_user_by_email()
            .returning(|_, _, _| Ok(None));
        mock.expect_find_user_by_username()
            .returning(|_, _, _| Ok(Some(User::fake().to_response()))); // <- สมมุติว่ามี method fake()

        let service = CreateUserServiceImpl::new(Arc::new(mock));

        let payload = build_payload(Some("mika@example.com".into()), "duplicate");
        let apporg = build_apporg(true, false); // username must be unique

        let res = service.execute(apporg, payload).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, 400);
    }

    #[tokio::test]
    async fn test_create_success() {
        let mut mock = MockCreateUserRepository::new();
        let email = Some("mika@example.com".into());
        let username = "mikatan";

        mock.expect_find_user_by_email()
            .returning(|_, _, _| Ok(None));
        mock.expect_find_user_by_username()
            .returning(|_, _, _| Ok(None));
        mock.expect_hash_password()
            .returning(|_| Ok("hashed".into()));

        let user = User::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            username.into(),
            "hashed".into(),
            email.clone(),
        );

        let user_clone = user.clone();
        mock.expect_new_user()
            .returning(move |_, _, _| user_clone.clone());

        let uorg = UserOrganization::new(build_apporg(true, false), user.clone());
        mock.expect_new_user_organization()
            .returning(move |_, _| uorg.clone());

        mock.expect_create_user().returning(|_| Ok(()));

        let service = CreateUserServiceImpl::new(Arc::new(mock));

        let payload = build_payload(email.clone(), username);
        let apporg = build_apporg(true, false);

        let res = service.execute(apporg, payload).await;
        assert!(res.is_ok());

        let user_response = res.unwrap();
        assert_eq!(user_response.username, username);
        assert_eq!(user_response.email, email);
    }
}
