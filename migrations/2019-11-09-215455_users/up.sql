CREATE TABLE users (
                       id          SERIAL PRIMARY KEY,
                       name        TEXT      NOT NULL,
                       email       TEXT      NOT NULL,
                       password    TEXT      NOT NULL,
                       quick_token TEXT      NOT NULL UNIQUE,
                       joined      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       is_admin    BOOLEAN   NOT NULL DEFAULT FALSE
);
CREATE UNIQUE INDEX ON users(is_admin)
    WHERE is_admin = TRUE;