CREATE OR REPLACE FUNCTION prune() RETURNS TRIGGER AS
$data$
BEGIN
    DELETE FROM data WHERE hash NOT IN (SELECT hash FROM files);
    RETURN NEW;
END
$data$ LANGUAGE plpgsql;