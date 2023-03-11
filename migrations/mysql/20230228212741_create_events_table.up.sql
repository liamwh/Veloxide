CREATE TABLE events (
  aggregate_type VARCHAR(255) NOT NULL,
  aggregate_id VARCHAR(255) NOT NULL,
  sequence bigint CHECK (sequence >= 0) NOT NULL,
  event_type VARCHAR(255) NOT NULL,
  event_version VARCHAR(255) NOT NULL,
  payload json NOT NULL,
  metadata json NOT NULL,
  PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);