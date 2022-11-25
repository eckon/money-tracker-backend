CREATE TABLE auth_user (
  -- currently this is the "real" id, should be done differently and generated differently
  access_token  VARCHAR NOT NULL PRIMARY KEY,
  -- for now it is discord id - later probably should change
  id            VARCHAR NOT NULL,
  avatar        VARCHAR,
  username      VARCHAR NOT NULL,
  discriminator VARCHAR NOT NULL
);
