-- Your SQL goes here
ALTER TABLE files
    ALTER COLUMN directory
        DROP NOT NULL;