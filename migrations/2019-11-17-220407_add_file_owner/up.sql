-- Your SQL goes here
ALTER TABLE files
    ADD COLUMN owner INTEGER NOT NULL REFERENCES users (id) DEFAULT 1;