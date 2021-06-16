insert into "ClientCard" (first_name, last_name, patronymic, phone_num, addr_city, addr_street, addr_postal, discount_rate)
values ($1, $2, $3, $4, $5, $6, $7, $8)
returning *;