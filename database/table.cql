CREATE TABLE axcelium.users (
  user_id           UUID,
  organization_id   UUID,
  application_id    UUID,
  username          TEXT,
  email             TEXT,
  hashed_password   TEXT,
  created_at        TIMESTAMP,
  updated_at        TIMESTAMP,
  is_active         BOOLEAN,
  is_verified       BOOLEAN,
  is_locked         BOOLEAN,
  last_login        TIMESTAMP,
  mfa_enabled       BOOLEAN,
  deactivated_at    TIMESTAMP,
  PRIMARY KEY ((organization_id, application_id), user_id)
);
CREATE TABLE axcelium.applications (
  application_id  UUID,
  organization_id UUID,
  name            TEXT,
  description     TEXT,
  client_id       UUID,
  client_secret   TEXT,
  config          TEXT,
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


CREATE TABLE axcelium.applications_organization_by_client_id (
  client_id                     UUID PRIMARY KEY,
  application_id                UUID,
  organization_id               UUID,
  client_secret                 TEXT,
  organization_name             TEXT,
  organization_slug             TEXT,
  application_name              TEXT,
  application_config            TEXT,
  application_description       TEXT,
  is_active                     BOOLEAN,
  created_at                    TIMESTAMP,
  updated_at                    TIMESTAMP
);
CREATE TABLE axcelium.users_by_email (
  email           TEXT,
  organization_id UUID,
  application_id  UUID,
  user_id         UUID,
  username        TEXT,
  password_hash   TEXT,
  created_at      TIMESTAMP,
  updated_at      TIMESTAMP,
  is_active       BOOLEAN,
  is_verified     BOOLEAN,
  is_locked       BOOLEAN,
  last_login      TIMESTAMP,
  mfa_enabled     BOOLEAN,
  deactivated_at  TIMESTAMP,
  PRIMARY KEY ((email, application_id, organization_id), user_id)
);

CREATE TABLE axcelium.users_by_username (
  username        TEXT,
  organization_id UUID,
  application_id  UUID,
  email           TEXT,
  user_id         UUID,
  password_hash   TEXT,
  created_at      TIMESTAMP,
  updated_at      TIMESTAMP,
  is_active       BOOLEAN,
  is_verified     BOOLEAN,
  is_locked       BOOLEAN,
  last_login      TIMESTAMP,
  mfa_enabled     BOOLEAN,
  deactivated_at  TIMESTAMP,
  PRIMARY KEY ((username, application_id, organization_id), user_id)
);

CREATE TABLE axcelium.user_organizations (
  organization_id               UUID,
  user_id                       UUID,
  role                          TEXT,
  username                      TEXT,
  user_email                    TEXT,
  organization_name             TEXT,
  organization_slug             TEXT,
  contact_email                 TEXT,
  joined_at                     TIMESTAMP,
  PRIMARY KEY ((organization_id), user_id)
);
CREATE TABLE axcelium.user_organizations_by_user (
  user_id                       UUID,
  organization_id               UUID,
  role                          TEXT,
  username                      TEXT,
  user_email                    TEXT,
  organization_name             TEXT,
  organization_slug             TEXT,
  contact_email                 TEXT,
  joined_at                     TIMESTAMP,
  PRIMARY KEY ((user_id), organization_id)
);