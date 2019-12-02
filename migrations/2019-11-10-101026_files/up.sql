CREATE TABLE files(
                      id        SERIAL PRIMARY KEY,
                      filename  TEXT      NOT NULL,
                      hash      SERIAL    NOT NULL REFERENCES content (hash),
                      created   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                      updated   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                      directory INTEGER   NOT NULL REFERENCES directories (id),
                      public    BOOLEAN   NOT NULL
);