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
  -- this needs to be handled by the backend, but at least it gets created
  -- also needs to be passed by the FE (maybe use another field as this is creation)
  creation_date DATE NOT NULL DEFAULT CURRENT_DATE,
  description TEXT,
  tags VARCHAR[],

  CONSTRAINT account_id
    FOREIGN KEY(account_id)
      REFERENCES account(id)
        ON DELETE CASCADE
);
