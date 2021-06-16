UPDATE "ClientCard"
SET first_name = $2, last_name = $3, patronymic = $4, phone_num = $5, addr_city = $6,
    addr_street = $7, addr_postal = $8, discount_rate = $9
WHERE card_id = $1
RETURNING *;