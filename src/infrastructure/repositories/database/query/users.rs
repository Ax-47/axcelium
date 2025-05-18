pub const INSERT_USER: &str = r#"
    INSERT INTO axcelium.users (
        user_id, organization_id, application_id,
        username, email, hashed_password, locked_at,
        created_at, updated_at,
        is_active, is_verified, is_locked, mfa_enabled, last_login, deactivated_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,? ) IF NOT EXISTS
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
pub const QUERY_FIND_ALL_USERS_PAGINATED: &str = r#"
    SELECT user_id, organization_id, application_id, username,
        email, created_at, updated_at, is_active, is_verified, is_locked,
        last_login, mfa_enabled, deactivated_at
    FROM axcelium.users_by_app
    WHERE organization_id = ? AND application_id = ?
"#;
pub const QUERY_FIND_USER: &str = r#"
    SELECT user_id, organization_id, application_id, username,
        email, created_at, updated_at, is_active, is_verified, is_locked,
        last_login, mfa_enabled, deactivated_at
    FROM axcelium.users
    WHERE organization_id = ? AND application_id = ? AND user_id = ?
"#;

pub const QUERY_FIND_RAW_USER: &str = r#"
    SELECT user_id, organization_id, application_id, username,
        email, created_at, updated_at, is_active, is_verified, is_locked,
        last_login, mfa_enabled, deactivated_at, hashed_password
    FROM axcelium.users
    WHERE organization_id = ? AND application_id = ? AND user_id = ?
"#;
// Static queries
pub const DELETE_USERS_BY_EMAIL: &str = "
    DELETE FROM axcelium.users_by_email
    WHERE organization_id = :organization_id AND application_id = :application_id AND email = :email ";

pub const DELETE_USERS_BY_USERNAME: &str = "
    DELETE FROM axcelium.users_by_username
    WHERE organization_id = :organization_id AND application_id = :application_id AND username = :username";

pub const INSERT_USERS_BY_USERNAME_SEC: &str = "
    INSERT INTO axcelium.users_by_username (
        username, organization_id, application_id,
        email, user_id, hashed_password,
        created_at, updated_at,
        is_active, is_verified, is_locked,
        last_login, mfa_enabled, deactivated_at
    ) VALUES (
        :username, :organization_id, :application_id,
        :email, :user_id, :hashed_password,
        :created_at, :updated_at,
        :is_active, :is_verified, :is_locked,
        :last_login, :mfa_enabled, :deactivated_at
    )";

pub const INSERT_USERS_BY_EMAIL_SEC: &str = "
    INSERT INTO axcelium.users_by_email (
        username, organization_id, application_id,
        email, user_id, hashed_password,
        created_at, updated_at,
        is_active, is_verified, is_locked,
        last_login, mfa_enabled, deactivated_at
    ) VALUES (
        :username, :organization_id, :application_id,
        :email, :user_id, :hashed_password,
        :created_at, :updated_at,
        :is_active, :is_verified, :is_locked,
        :last_login, :mfa_enabled, :deactivated_at
    )";

pub fn update_users_by_email(set_clauses: &[&str]) -> String {
    format!(
        "UPDATE axcelium.users_by_email SET {}, updated_at = :updated_at \
        WHERE organization_id = :organization_id AND application_id = :application_id AND email = :email AND user_id = :user_id" ,
        set_clauses.join(", ")
    )
}

pub fn update_users_by_username(set_clauses: &[&str]) -> String {
    format!(
        "UPDATE axcelium.users_by_username SET {}, updated_at = :updated_at \
        WHERE organization_id = :organization_id AND application_id = :application_id AND username = :username AND user_id = :user_id" ,
        set_clauses.join(", ")
    )
}
// Dynamic queries
pub fn update_users_query(set_clauses: &[&str]) -> String {
    format!(
        "UPDATE axcelium.users SET {}, updated_at = :updated_at \
        WHERE organization_id = :organization_id AND application_id = :application_id AND user_id = :user_id",
        set_clauses.join(", ")
    )
}

pub fn update_user_org_query(set_clauses: &[&str]) -> String {
    format!(
        "UPDATE axcelium.user_organizations SET {} \
        WHERE organization_id = :organization_id AND user_id = :user_id",
        set_clauses.join(", ")
    )
}

pub fn update_user_org_by_user_query(set_clauses: &[&str]) -> String {
    format!(
        "UPDATE axcelium.user_organizations_by_user SET {} \
        WHERE organization_id = :organization_id AND user_id = :user_id",
        set_clauses.join(", ")
    )
}
pub const DELETE_USERS: &str = r#"
    DELETE FROM axcelium.users 
    WHERE user_id = :user_id AND organization_id = :organization_id AND application_id = :application_id
"#;
pub const DELETE_USER_ORG: &str = r#"
    DELETE FROM axcelium.user_organizations 
    WHERE organization_id = :organization_id AND user_id = :user_id
"#;

pub const DELETE_USER_ORG_BY_USER: &str = r#"
    DELETE FROM axcelium.user_organizations_by_user 
    WHERE organization_id = :organization_id AND user_id = :user_id
"#;
