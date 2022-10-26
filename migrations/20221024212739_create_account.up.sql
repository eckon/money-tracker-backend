CREATE TABLE account (
  id UUID NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TYPE account_entry_kind AS ENUM ('cost', 'payment');

CREATE TABLE account_entry (
  id UUID NOT NULL PRIMARY KEY,
  account_id UUID NOT NULL,
  kind account_entry_kind NOT NULL,
  amount BIGINT NOT NULL,
  CONSTRAINT account_id
    FOREIGN KEY(account_id)
      REFERENCES account(id)
);
