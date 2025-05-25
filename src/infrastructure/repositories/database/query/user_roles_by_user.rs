pub const ASSIGN_ROLE_TO_USER: &str = r#"
INSERT INTO axcelium.user_roles_by_user (
    organization_id, application_id, user_id,
    role_id, assigned_at
) VALUES (
    :organization_id, :application_id, :user_id,
    :role_id, toTimestamp(now())
);"#;

pub const LIST_ROLES_OF_USER: &str = r#"
SELECT role_id, assigned_at
FROM axcelium.user_roles_by_user
WHERE organization_id = ? AND application_id = ? AND user_id = ?;
"#;