INSERT INTO "Manufacturer" (contract_id, contract_sign_date, contract_end_date, manufacturer_name, country, addr_city, addr_street,
                            addr_postal, tel_num)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING *;