-- This checks if, in the case of a non-null parent ID, the parent directory has the same owner.
-- owner is $1, parent is $2
SELECT (SELECT owner from directories WHERE id = $2) = $1;