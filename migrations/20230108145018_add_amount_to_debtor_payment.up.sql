ALTER TABLE debt
-- TODO: remove default after it is the only needed part (percentage is dropped)
  ADD COLUMN amount BIGINT NOT NULL DEFAULT 0;

ALTER TABLE debt
  ALTER COLUMN percentage SET DEFAULT 0;
