pub const INSERT_REFRESH_TOKEN :&str=r#"INSERT INTO refresh_tokens ( 
    token_id,
    application_id,
    organization_id,
    user_id,
    encrypted_token_secret,
    token_version,
    parent_version,
    issued_at,
    expires_at,
    revoked
) VALUES (
    :token_id,
    :application_id,
    :organization_id,
    :user_id,
    :encrypted_token_secret,
    :token_version,
    :parent_version,
    :issued_at,
    :expires_at,
    :revoked
) USING TTL 2592000;"#;