CREATE TABLE account (
  id   VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,

  PRIMARY KEY (id)
);

CREATE TABLE auth_user (
  -- currently this is the "real" id, should be done differently and generated differently
  access_token  VARCHAR(255) NOT NULL,
  -- for now it is discord id - later probably should change
  id            VARCHAR(255) NOT NULL,
  avatar        VARCHAR(255),
  username      VARCHAR(255) NOT NULL,
  discriminator VARCHAR(255) NOT NULL,
  creation_date DATE         DEFAULT (CURRENT_DATE),

  PRIMARY KEY (access_token)
);

CREATE TABLE cost (
  id          VARCHAR(255) NOT NULL,
  account_id  VARCHAR(255) NOT NULL,
  amount      BIGINT       NOT NULL,
  event_date  DATE         NOT NULL,
  description TEXT,
  tags        JSON,

  PRIMARY KEY (id),

  -- foreign key not allowed because of Online DDL
  KEY account_id_idx (account_id)
);

CREATE TABLE payment (
  id                VARCHAR(255) NOT NULL,
  payer_account_id  VARCHAR(255) NOT NULL,
  lender_account_id VARCHAR(255) NOT NULL,
  amount            BIGINT NOT NULL,
  event_date        DATE   NOT NULL,
  description       TEXT,

  PRIMARY KEY (id),

  -- foreign key not allowed because of Online DDL
  KEY player_account_id_idx (payer_account_id),
  KEY lender_account_id_idx (lender_account_id)
);

CREATE TABLE debt (
  id                 VARCHAR(255) NOT NULL,
  debtor_account_id  VARCHAR(255) NOT NULL,
  cost_id            VARCHAR(255) NOT NULL,
  amount             BIGINT       NOT NULL,

  PRIMARY KEY (id),

  -- foreign key not allowed because of Online DDL
  KEY debtor_account_id_idx (debtor_account_id),
  KEY cost_id_idx (cost_id)
);
