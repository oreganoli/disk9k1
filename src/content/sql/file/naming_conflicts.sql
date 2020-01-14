SELECT NOT EXISTS(
        SELECT id
        FROM files
        WHERE name = $1
          AND owner = $2
          AND directory IS NOT DISTINCT FROM $3
    );