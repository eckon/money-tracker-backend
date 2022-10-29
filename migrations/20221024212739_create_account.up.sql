CREATE TABLE account (
  id   UUID    NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE payment (
  id                UUID   NOT NULL PRIMARY KEY,
  payer_account_id  UUID   NOT NULL,
  lender_account_id UUID   NOT NULL,
  amount            BIGINT NOT NULL,
  event_date        DATE   NOT NULL,
  description       TEXT,

  CONSTRAINT payer_account_id
    FOREIGN KEY(payer_account_id)
      REFERENCES account(id)
        ON DELETE CASCADE,

  CONSTRAINT lender_account_id
    FOREIGN KEY(lender_account_id)
      REFERENCES account(id)
        ON DELETE CASCADE
);

CREATE TABLE cost (
  id          UUID      NOT NULL PRIMARY KEY,
  account_id  UUID      NOT NULL,
  amount      BIGINT    NOT NULL,
  event_date  DATE      NOT NULL,
  description TEXT,
  tags        VARCHAR[],

  CONSTRAINT account_id
    FOREIGN KEY(account_id)
      REFERENCES account(id)
        ON DELETE CASCADE
);

CREATE TABLE debt (
  id                 UUID     NOT NULL PRIMARY KEY,
  debtor_account_id  UUID     NOT NULL,
  cost_id            UUID     NOT NULL,
  percentage         SMALLINT NOT NULL,

  CONSTRAINT debtor_account_id
    FOREIGN KEY(debtor_account_id)
      REFERENCES account(id)
        ON DELETE CASCADE,

  CONSTRAINT cost_id
    FOREIGN KEY(cost_id)
      REFERENCES cost(id)
        ON DELETE CASCADE
);
