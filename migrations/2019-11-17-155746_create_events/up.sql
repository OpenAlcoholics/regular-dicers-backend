-- Your SQL goes here
CREATE TABLE IF NOT EXISTS events
(
    id        SERIAL UNIQUE PRIMARY KEY,
    chat_id   Integer   NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    active    bool      NOT NULL DEFAULT false,
    UNIQUE (chat_id, timestamp),
    UNIQUE (chat_id, active),
    FOREIGN KEY (chat_id) REFERENCES chats (id)
)
