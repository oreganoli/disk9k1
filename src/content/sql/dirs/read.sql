SELECT id, name, owner, parent
FROM directories
WHERE id = $1;