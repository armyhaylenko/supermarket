INSERT INTO "Product" (product_name, descr, category_id)
VALUES ($1, $2, $3)
RETURNING *;