SELECT *
FROM "Waybill"
WHERE product_upc = $1 AND waybill_date BETWEEN $2 AND $3