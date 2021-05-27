UPDATE "OwnedProduct"
SET product_id = $2, sale_price = $3, units_in_stock = $4, is_on_sale = $5, on_sale_product_upc = $6
WHERE product_upc = $1