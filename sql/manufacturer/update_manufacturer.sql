UPDATE "Manufacturer"
SET contract_id = $2, contract_sign_date = $3, contract_end_date = $4, manufacturer_name = $5, country = $6, addr_city = $7, addr_street = $8, addr_postal = $9, tel_num = $10
WHERE manufacturer_id = $1
