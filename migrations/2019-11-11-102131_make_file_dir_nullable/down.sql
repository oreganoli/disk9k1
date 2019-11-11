-- This file should undo anything in `up.sql`
ALTER TABLE files
    ALTER COLUMN directory
        SET NOT NULL;