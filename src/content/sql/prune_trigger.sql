CREATE TRIGGER prune_trigger
    AFTER DELETE
    ON files
EXECUTE PROCEDURE prune();