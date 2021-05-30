INSERT INTO "OwnedProduct" (product_upc, product_id, sale_price, units_in_stock, is_on_sale, on_sale_product_upc)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;