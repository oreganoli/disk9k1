SELECT NOT EXISTS(
        SELECT id
        FROM files
        WHERE filename = $1
          AND owner = $2
          AND directory IS NOT DISTINCT FROM $3
    );