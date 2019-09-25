-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ingredients
(
    id   integer UNIQUE PRIMARY KEY,
    name varchar(64) UNIQUE NOT NULL
);
