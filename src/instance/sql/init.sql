CREATE TABLE IF NOT EXISTS instance
(
    name        TEXT NOT NULL PRIMARY KEY,
    description TEXT,
    size_limit  BIGINT
);
