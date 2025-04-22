CREATE TABLE axcelium.users (
  user_id         UUID,
  organization_id UUID,
  application_id  UUID,
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
  PRIMARY KEY ((organization_id, application_id), user_id)
);
CREATE TABLE axcelium.applications (
  application_id  UUID,
  organization_id UUID,
  name            TEXT,
  description     TEXT,
  client_id       UUID,
  client_secret   TEXT,
  created_at      TIMESTAMP,
  updated_at      TIMESTAMP,
  PRIMARY KEY (organization_id, application_id)
);

CREATE TABLE axcelium.organizations (
  organization_id UUID PRIMARY KEY,
  name            TEXT,
  slug            TEXT,
  contact_email   TEXT,
  is_active       BOOLEAN,
  created_at      TIMESTAMP,
  updated_at      TIMESTAMP
);