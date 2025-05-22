pub const INSERT_REFRESH_TOKEN: &str = r#"
INSERT INTO axcelium.refresh_tokens (
    token_id, application_id, organization_id, user_id,
    encrypted_token_secret, token_version, parent_version,
    issued_at, expires_at, revoked
) VALUES (
    :token_id, :application_id, :organization_id,
    :user_id, :encrypted_token_secret, :token_version,
    :parent_version, :issued_at, :expires_at, :revoked
) USING TTL 2592000;"#;
pub const UPDATE_REFRESH_TOKEN: &str = r#"
UPDATE axcelium.refresh_tokens USING TTL 2592000 SET
    encrypted_token_secret = :encrypted_token_secret,
    token_version = :token_version,
    parent_version = :parent_version,
    issued_at = :issued_at,
    expires_at = :expires_at,
    revoked = :revoked
WHERE
    token_id = :token_id AND
    application_id = :application_id AND
    organization_id = :organization_id;"#;
pub const QUERY_REFRESH_TOKEN: &str = r#"
SELECT
    application_id, organization_id, user_id, encrypted_token_secret,
    parent_version, issued_at, expires_at, revoked
FROM axcelium.refresh_tokens_with_token_version
WHERE organization_id = ? AND application_id = ?AND token_id = ? AND token_version=?;
"#;

pub const REVOKE_REFRESH_TOKEN: &str = r#"
    UPDATE axcelium.refresh_tokens SET revoked=true
    WHERE organization_id = ? AND application_id = ? AND token_id = ?;
"#;
