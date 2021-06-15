update "Sale"
set price = $3,
    qty   = $4
where receipt_id = $1 and product_upc = $2;
