pub const INCREASE_USER: &str = r#"
UPDATE axcelium.user_count_by_app
SET user_count = user_count + 1
WHERE organization_id = ? AND application_id = ?"#;

pub const DECREASE_USER: &str = r#"
UPDATE axcelium.user_count_by_app
SET user_count = user_count - 1
WHERE organization_id = ? AND application_id = ?"#;
pub const SELECT_USER_COUNT: &str = r#"
SELECT user_count
FROM axcelium.user_count_by_app
WHERE organization_id = ? AND application_id = ?;
"#;
pub const INSERT_USER: &str = r#"
    INSERT INTO axcelium.users (
        user_id, organization_id, application_id,
        username, email, hashed_password, locked_at,
        created_at, updated_at,
        is_active, is_verified, is_locked, mfa_enabled, last_login, deactivated_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) IF NOT EXISTS
"#;

pub const QUERY_FIND_USER_BY_EMAIL: &str = r#"
    SELECT username, user_id, email
    FROM axcelium.users_by_email_app_org
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
        last_login, mfa_enabled, deactivated_at,locked_at
    FROM axcelium.users_by_app
    WHERE organization_id = ? AND application_id = ?
"#;
pub const QUERY_FIND_USER: &str = r#"
    SELECT user_id, organization_id, application_id, username,
        email, created_at, updated_at, is_active, is_verified, is_locked,locked_at,
        last_login, mfa_enabled, deactivated_at
    FROM axcelium.users
    WHERE organization_id = ? AND application_id = ? AND user_id = ?
"#;

pub const QUERY_FIND_RAW_USER: &str = r#"
    SELECT user_id, organization_id, application_id, username,
        email, created_at, updated_at, is_active, is_verified, is_locked,locked_at,
        last_login, mfa_enabled, deactivated_at, hashed_password
    FROM axcelium.users
    WHERE organization_id = ? AND application_id = ? AND user_id = ?
"#;

pub const UPDATE_USER: &str = r#"
    UPDATE axcelium.users SET
        username = :username, email = :email, hashed_password = :hashed_password,
        locked_at = :locked_at, created_at = :created_at, updated_at = :updated_at,
        is_active = :is_active, is_verified = :is_verified, is_locked = :is_locked,
        mfa_enabled = :mfa_enabled, last_login = :last_login, deactivated_at = :deactivated_at
    WHERE organization_id = ? AND application_id = ? AND user_id = ?
"#;

pub const DELETE_USER: &str = r#"
    DELETE FROM axcelium.users
    WHERE user_id = :user_id AND organization_id = :organization_id AND application_id = :application_id
"#;

pub const BAN_USER: &str = r#"
    UPDATE axcelium.users SET is_locked=true, locked_at= toTimestamp(now()), updated_at =  toTimestamp(now())
    WHERE organization_id = :organization_id AND application_id = :application_id AND user_id = :user_id;
"#;

pub const UNBAN_USER: &str = r#"
    UPDATE axcelium.users SET is_locked=false, updated_at =  toTimestamp(now())
    WHERE organization_id = :organization_id AND application_id = :application_id AND user_id = :user_id;
"#;

pub const DISABLE_MFA_USER: &str = r#"
    UPDATE axcelium.users SET mfa_enabled=false, updated_at =  toTimestamp(now())
    WHERE organization_id = :organization_id AND application_id = :application_id AND user_id = :user_id;
"#;