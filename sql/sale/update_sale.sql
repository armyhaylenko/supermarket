begin;
update "Sale"
set price = $3,
    qty   = $4
where receipt_id = $1 and product_upc = $2;

update "Receipt"
set receipt_sum = (select sum(qty * price) as purchase_sum
                         from "Sale"
                         where "Sale".receipt_id = $1
                         group by "Sale".receipt_id)
where receipt_id = $1;
commit;