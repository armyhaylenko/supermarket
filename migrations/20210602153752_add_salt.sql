-- Add migration script here

ALTER TABLE users
ADD COLUMN IF NOT EXISTS salt character(7) NOT NULL