-- Add migration script here

ALTER TABLE users
ALTER COLUMN salt TYPE character(32);
