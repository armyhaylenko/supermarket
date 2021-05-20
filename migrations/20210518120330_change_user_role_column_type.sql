-- Add migration script here

ALTER TABLE users
ALTER COLUMN user_role TYPE character(7);

DROP TYPE user_role;