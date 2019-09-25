-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chat_users
(
    id       SERIAL UNIQUE PRIMARY KEY,
    chat_id  Integer NOT NULL,
    user_id  Integer NOT NULL,
    spamming boolean NOT NULL DEFAULT false,
    muted    boolean NOT NULL DEFAULT false,
    admin    boolean NOT NULL DEFAULT false,
    FOREIGN KEY (chat_id) REFERENCES chats (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    UNIQUE (chat_id, user_id),
    constraint admin_cant_be_muted
        check ((admin = true AND muted = false) OR (admin = false))
)
