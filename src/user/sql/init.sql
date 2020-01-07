CREATE TABLE IF NOT EXISTS users
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL UNIQUE CHECK (length(name) > 0 AND name ~ '^[^<>`~!\\@#}/$%:;)(^{&*=|''+\s]+$'),
    email    VARCHAR NOT NULL UNIQUE CHECK (email ~
                                            '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$'),
    password VARCHAR NOT NULL,
    is_admin BOOLEAN NOT NULL
);