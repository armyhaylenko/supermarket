INSERT INTO "Receipt" (receipt_date, receipt_sum, "VAT", client_card_id) VALUES ($1, $2, $3, $4) RETURNING *;