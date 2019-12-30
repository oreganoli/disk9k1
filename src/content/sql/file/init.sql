CREATE TABLE IF NOT EXISTS files
(
    id        SERIAL PRIMARY KEY,
    filename  TEXT    NOT NULL CHECK (filename ~ '(^\.?[^.\r\n\t\\/:"|?*<>]+[^\r\n\t\\/:"|?*<>]*$)'),
    hash      BIGINT  NOT NULL REFERENCES data (hash),
    owner     INTEGER NOT NULL REFERENCES users (id),
    public    BOOLEAN NOT NULL,
    directory INTEGER NULL REFERENCES directories (id)
);