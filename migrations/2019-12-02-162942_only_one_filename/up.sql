ALTER TABLE files
    ADD CONSTRAINT action_unique_fn UNIQUE (filename);