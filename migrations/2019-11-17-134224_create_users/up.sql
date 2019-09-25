-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    id                       SERIAL UNIQUE PRIMARY KEY,
    username                 Text,
    first_name               Text    NOT NULL,
    last_name                Text,
    telegram_id              Integer NOT NULL UNIQUE,
    is_bot                   boolean NOT NULL default false,
    has_private_conversation boolean NOT NULL default false
)
