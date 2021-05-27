UPDATE "Product"
SET product_name = $2, descr = $3, category_id = $4
WHERE product_id = $1
