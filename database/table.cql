CREATE TABLE axcelium.users (
  organization_id UUID,
  application_id  UUID,
  id              UUID,
  username        TEXT,
  email           TEXT,
  password_hash   TEXT,
  created_at      TIMESTAMP,
  updated_at      TIMESTAMP,
  is_active       BOOLEAN,
  is_verified     BOOLEAN,
  is_locked       BOOLEAN,
  last_login      TIMESTAMP,
  mfa_enabled     BOOLEAN,
  deactivated_at  TIMESTAMP,
  PRIMARY KEY ((organization_id, application_id), id)
);