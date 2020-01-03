UPDATE directories
SET parent = $2
WHERE id = $1;