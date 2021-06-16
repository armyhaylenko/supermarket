SELECT *
FROM "Product" INNER JOIN "Category" C on C.category_id = "Product".category_id
WHERE category_name = $1
ORDER BY product_name