CREATE TABLE IF NOT EXISTS instance
(
    name        TEXT NOT NULL PRIMARY KEY,
    description TEXT,
    size_limit BIGINT NOT NULL CHECK (size_limit >= 0)
);
