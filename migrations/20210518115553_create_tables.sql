-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role AS ENUM ('manager', 'cashier');

CREATE TABLE users (
                       id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
                       username character varying(150) NOT NULL UNIQUE,
                       email character varying(200) NOT NULL UNIQUE,
                       user_role user_role NOT NULL,
                       password_hash character(256) NOT NULL,
                       full_name character varying(200) NULL,
                       image_link character varying(200) NULL,
                       created_at timestamp NOT NULL DEFAULT current_timestamp,
                       updated_at timestamp NOT NULL DEFAULT current_timestamp
);
