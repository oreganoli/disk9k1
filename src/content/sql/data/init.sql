CREATE TABLE IF NOT EXISTS data
(
    hash     BIGINT PRIMARY KEY,
    mimetype TEXT  NOT NULL,
    data     BYTEA NOT NULL
);