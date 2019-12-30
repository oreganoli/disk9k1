SELECT id, name, owner, parent
FROM directories
WHERE parent = $1;