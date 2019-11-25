-- This file should undo anything in `up.sql`

ALTER TABLE directories
    ADD COLUMN public BOOLEAN NOT NULL DEFAULT false;