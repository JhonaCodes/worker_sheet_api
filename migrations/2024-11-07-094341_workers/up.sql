-- Your SQL goes here
CREATE TABLE workers (
                         id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid()::text,
                         name VARCHAR NOT NULL,
                         email VARCHAR NOT NULL UNIQUE
);