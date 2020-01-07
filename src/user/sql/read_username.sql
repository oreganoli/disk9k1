SELECT id, name, email, password, is_admin
FROM users
WHERE name = $1;