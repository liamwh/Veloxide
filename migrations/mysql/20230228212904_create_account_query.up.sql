CREATE TABLE account_query (
  view_id VARCHAR(255) NOT NULL,
  version bigint CHECK (version >= 0) NOT NULL,
  payload json NOT NULL,
  PRIMARY KEY (view_id)
);