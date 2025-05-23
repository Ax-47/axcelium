CREATE TABLE axcelium.users (
  user_id UUID,
  organization_id UUID,
  application_id UUID,
  username TEXT,
  email TEXT,
  hashed_password TEXT,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  is_active BOOLEAN,
  is_verified BOOLEAN,
  is_locked BOOLEAN,
  last_login TIMESTAMP,
  mfa_enabled BOOLEAN,
  deactivated_at TIMESTAMP,
  locked_at TIMESTAMP,
  PRIMARY KEY ((user_id, organization_id, application_id))
);
CREATE MATERIALIZED VIEW users_by_app AS
SELECT *
FROM axcelium.users
WHERE organization_id IS NOT NULL
  AND application_id IS NOT NULL
  AND username IS NOT NULL PRIMARY KEY (
    (organization_id, application_id),
    created_at,
    user_id
  ) WITH CLUSTERING
ORDER BY (created_at DESC);
CREATE MATERIALIZED VIEW users_by_username AS
SELECT *
FROM axcelium.users
WHERE organization_id IS NOT NULL
  AND application_id IS NOT NULL
  AND username IS NOT NULL PRIMARY KEY (
    (organization_id, application_id, username),
    user_id
  );
CREATE MATERIALIZED VIEW users_by_email_app_org AS
SELECT *
FROM axcelium.users
WHERE organization_id IS NOT NULL
  AND application_id IS NOT NULL
  AND email IS NOT NULL PRIMARY KEY ((organization_id, application_id, email));
CREATE TABLE axcelium.user_count_by_app (
  organization_id UUID,
  application_id UUID,
  user_count COUNTER,
  PRIMARY KEY ((organization_id, application_id))
);
CREATE TABLE axcelium.applications (
  application_id UUID,
  organization_id UUID,
  name TEXT,
  description TEXT,
  client_id UUID,
  encrypted_client_secret TEXT,
  config TEXT,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  PRIMARY KEY ((organization_id, application_id))
);
CREATE INDEX applications_client_id_sec_ix ON axcelium.users((organization_id, application_id, email));
CREATE TABLE axcelium.organizations (
  organization_id UUID PRIMARY KEY,
  name TEXT,
  slug TEXT,
  contact_email TEXT,
  is_active BOOLEAN,
  created_at TIMESTAMP,
  updated_at TIMESTAMP
);
CREATE TABLE axcelium.applications_organization_by_client_id (
  client_id UUID PRIMARY KEY,
  application_id UUID,
  organization_id UUID,
  encrypted_client_secret TEXT,
  organization_name TEXT,
  organization_slug TEXT,
  application_name TEXT,
  application_config TEXT,
  application_description TEXT,
  contact_email TEXT,
  is_active BOOLEAN,
  created_at TIMESTAMP,
  updated_at TIMESTAMP
);
CREATE TABLE refresh_tokens (
  token_id UUID,
  application_id UUID,
  organization_id UUID,
  user_id UUID,
  encrypted_token_secret TEXT,
  token_version TEXT,
  parent_version TEXT,
  issued_at TIMESTAMP,
  expires_at TIMESTAMP,
  revoked BOOLEAN,
  PRIMARY KEY ((organization_id, application_id, token_id))
) WITH default_time_to_live = 2592000;
CREATE MATERIALIZED VIEW refresh_tokens_with_token_version AS
SELECT *
FROM axcelium.refresh_tokens
WHERE organization_id IS NOT NULL
  AND application_id IS NOT NULL
  AND token_version IS NOT NULL
  AND token_id IS NOT NULL PRIMARY KEY (
    (
      organization_id,
      application_id,
      token_id,
      token_version
    )
  );
CREATE MATERIALIZED VIEW refresh_tokens_by_user AS
SELECT *
FROM axcelium.refresh_tokens
WHERE organization_id IS NOT NULL
  AND application_id IS NOT NULL
  AND user_id IS NOT NULL
  AND token_id IS NOT NULL
PRIMARY KEY (
  (organization_id, application_id, user_id),
  token_id
);
CREATE TABLE axcelium.roles_by_app (
  organization_id UUID,
  application_id UUID,
  role_id UUID,
  name TEXT,
  description TEXT,
  permissions SET<TEXT>,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  PRIMARY KEY ((organization_id, application_id), role_id)
);
CREATE TABLE axcelium.user_roles_by_user (
  organization_id UUID,
  application_id UUID,
  user_id UUID,
  role_id UUID,
  assigned_at TIMESTAMP,
  PRIMARY KEY ((organization_id, application_id, user_id), role_id)
);
CREATE TABLE axcelium.role_users_by_role (
  organization_id UUID,
  application_id UUID,
  role_id UUID,
  user_id UUID,
  assigned_at TIMESTAMP,
  PRIMARY KEY ((organization_id, application_id, role_id), user_id)
);