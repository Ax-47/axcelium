pub const INSERT_USER: &str = r#"
    INSERT INTO axcelium.users (
        user_id, organization_id, application_id,
        username, email, hashed_password,
        created_at, updated_at,
        is_active, is_verified, is_locked, mfa_enabled, last_login, deactivated_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_USER_BY_EMAIL: &str = r#"
    INSERT INTO axcelium.users_by_email (
        email, organization_id, application_id,
        user_id, username, hashed_password,
        created_at, updated_at,
        is_active, is_verified, is_locked,
        last_login, mfa_enabled, deactivated_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_USER_BY_USERNAME: &str = r#"
    INSERT INTO axcelium.users_by_username (
        username, organization_id, application_id,
        email, user_id, hashed_password,
        created_at, updated_at,
        is_active, is_verified, is_locked,
        last_login, mfa_enabled, deactivated_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_USER_ORGANIZATION: &str = r#"
    INSERT INTO axcelium.user_organizations (
        organization_id, user_id, role,
        username, user_email,
        organization_name, organization_slug, contact_email,
        joined_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_USER_ORG_BY_USER: &str = r#"
    INSERT INTO axcelium.user_organizations_by_user (
        user_id, organization_id, role,
        username, user_email,
        organization_name, organization_slug, contact_email,
        joined_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;
pub const QUERY_FIND_USER_BY_EMAIL: &str = r#"
    SELECT username, user_id, email
    FROM axcelium.users_by_email
    WHERE email = ? AND application_id = ? AND organization_id = ?
"#;

pub const QUERY_FIND_USER_BY_USERNAME: &str = r#"
    SELECT username, user_id, email
    FROM axcelium.users_by_username
    WHERE username = ? AND application_id = ? AND organization_id = ?
"#;