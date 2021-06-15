SELECT *
FROM "OwnedProduct"
    INNER JOIN "Product" P on P.product_id = "OwnedProduct".product_id
WHERE product_name = $1