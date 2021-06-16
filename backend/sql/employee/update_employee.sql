UPDATE "ShopEmployee"
SET first_name = $2, last_name = $3, patronymic = $4, position = $5, salary = $6, join_date = $7, phone_num = $8, addr_city = $9,
    addr_street = $10, addr_postal = $11
WHERE empl_id = $1
RETURNING *;