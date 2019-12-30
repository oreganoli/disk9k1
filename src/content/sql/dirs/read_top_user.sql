SELECT id, name, owner, parent
FROM directories
WHERE owner = $1
  AND parent IS NULL;