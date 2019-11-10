-- Your SQL goes here
CREATE TABLE instance
(
    name        TEXT   NOT NULL PRIMARY KEY,
    description TEXT   NOT NULL,
    size_limit  BIGINT NOT NULL
);

-- only one row of instance data is allowed
CREATE UNIQUE INDEX one_row
    ON instance ((name IS NOT NULL));