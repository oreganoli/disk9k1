SELECT NOT EXISTS(
        SELECT id
        FROM directories
        WHERE name = $1
          AND owner = $2
          AND parent IS NOT DISTINCT FROM $3
    );