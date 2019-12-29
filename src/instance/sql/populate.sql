DO
$do$
    BEGIN
        IF NOT EXISTS(SELECT * from instance) THEN
            INSERT INTO instance
            VALUES ('Disk9k1', 'Disk9k1 is a simple service for uploading and retrieving files.', 10485760);
        END IF;
    END
$do$