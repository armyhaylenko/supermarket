UPDATE "Category"
SET category_name = $2
WHERE category_id = $1
RETURNING *;