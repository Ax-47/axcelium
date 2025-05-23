pub const INSERT_ROLE_BY_APP: &str = r#"
INSERT INTO axcelium.roles_by_app (
    organization_id, application_id, role_id,
    name, description, permissions,
    created_at, updated_at
) VALUES (
    :organization_id, :application_id, :role_id,
    :name, :description, :permissions,
    :created_at, :updated_at
);"#;

pub const UPDATE_ROLE_BY_APP: &str = r#"
UPDATE axcelium.roles_by_app SET
    name = :name,
    description = :description,
    permissions = :permissions,
    updated_at = :updated_at
WHERE organization_id = :organization_id
  AND application_id = :application_id
  AND role_id = :role_id;"#;

pub const SELECT_ROLE_BY_ID: &str = r#"
SELECT
    name, description, permissions,
    created_at, updated_at
FROM axcelium.roles_by_app
WHERE organization_id = ? AND application_id = ? AND role_id = ?;
"#;

pub const DELETE_ROLE_BY_ID: &str = r#"
DELETE FROM axcelium.roles_by_app
WHERE organization_id = ? AND application_id = ? AND role_id = ?;
"#;
