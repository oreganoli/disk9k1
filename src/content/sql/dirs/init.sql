CREATE TABLE IF NOT EXISTS directories
(
    id     SERIAL PRIMARY KEY,
    name   TEXT    NOT NULL CHECK (name ~ '(^\.?[^.\r\n\t\\/:"|?*<>]+[^\r\n\t\\/:"|?*<>]*$)'),
    owner  INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    parent INTEGER NULL REFERENCES directories (id) ON DELETE CASCADE
);