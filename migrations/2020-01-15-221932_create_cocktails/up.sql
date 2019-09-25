-- Your SQL goes here
CREATE TABLE IF NOT EXISTS cocktails
(
    id        SERIAL UNIQUE PRIMARY KEY,
    name      varchar(64) NOT NULL,
    jumbo     boolean     NOT NULL,
    alcoholic boolean     NOT NULL
);
