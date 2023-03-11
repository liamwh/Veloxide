CREATE TABLE events (
  aggregate_type text NOT NULL,
  aggregate_id text NOT NULL,
  sequence bigint CHECK (sequence >= 0) NOT NULL,
  event_type text NOT NULL,
  event_version text NOT NULL,
  payload json NOT NULL,
  metadata json NOT NULL,
  PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);