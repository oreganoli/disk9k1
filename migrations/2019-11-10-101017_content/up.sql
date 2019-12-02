CREATE TABLE content(
                        hash             INTEGER PRIMARY KEY,
                        mimetype         TEXT      NOT NULL,
                        first_uploaded   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        times_uploaded   INTEGER   NOT NULL DEFAULT 0,
                        times_downloaded INTEGER   NOT NULL DEFAULT 0,
                        data             BYTEA     NOT NULL
);