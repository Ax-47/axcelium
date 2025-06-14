pub const ASSIGN_USER_TO_ROLE: &str = r#"
INSERT INTO axcelium.role_users_by_role (
    organization_id, application_id, role_id,
    user_id, assigned_at
) VALUES (
    :organization_id, :application_id, :role_id,
    :user_id, toTimestamp(now())
);"#;



pub const REMOVE_USER_FROM_ROLE: &str = r#"
DELETE FROM axcelium.role_users_by_role
WHERE organization_id = ? AND application_id = ? AND role_id = ? AND user_id = ?;
"#;

pub const LIST_USERS_IN_ROLE: &str = r#"
SELECT user_id, assigned_at
FROM axcelium.role_users_by_role
WHERE organization_id = ? AND application_id = ? AND role_id = ?;
"#;

