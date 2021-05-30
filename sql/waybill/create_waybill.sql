INSERT INTO "Waybill" (waybill_date, base_price, qty, product_upc, manufacturer_id, empl_id)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;