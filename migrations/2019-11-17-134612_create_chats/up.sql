-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chats
(
    id               SERIAL UNIQUE PRIMARY KEY,
    chat_type        SMALLINT     NOT NULL,
    telegram_id      Integer      NOT NULL UNIQUE,
    title            varchar(512) NOT NULL,
    description      Text,
    current_keyboard SMALLINT     NOT NULL DEFAULT 0,
    spam_detection   boolean      NOT NULL DEFAULT false,
    constraint valid_chat_type
        check ((chat_type >= 0) AND (chat_type <= 2)),
    constraint valid_keyboard_type
        check ((current_keyboard >= 0) AND (current_keyboard <= 2))
)
