-- Your SQL goes here
CREATE TABLE IF NOT EXISTS messages
(
    id                      SERIAL UNIQUE PRIMARY KEY,
    telegram_id             Integer   NOT NULL,
    chat_id                 Integer   NOT NULL,
    user_id                 Integer,
    timestamp               TIMESTAMP NOT NULL,
    reply_to_message_id     Integer,
    edit_timestamp          TIMESTAMP,
    text                    Text,
    caption                 Text,
    new_chat_member_ids     Integer[],
    left_chat_member_id     Integer,
    new_chat_title          Text,
    group_chat_created      boolean,
    supergroup_chat_created boolean,
    migrate_to_chat_id      Integer,
    migrate_from_chat_id    Integer,
    pinned_message_id       Integer,
    UNIQUE (chat_id, pinned_message_id),
    UNIQUE (chat_id, telegram_id),
    FOREIGN KEY (chat_id) REFERENCES chats (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
--     FOREIGN KEY (reply_to_message_id) REFERENCES messages (id),
--     FOREIGN KEY (pinned_message_id) REFERENCES messages (id),
--     FOREIGN KEY (migrate_from_chat_id) REFERENCES chats (id),
--     FOREIGN KEY (migrate_to_chat_id) REFERENCES chats (id),
--     FOREIGN KEY (left_chat_member_id) REFERENCES users (id),
    constraint reply_id_cant_be_id
        check (reply_to_message_id != id),
    constraint pinned_id_cant_be_id
        check (pinned_message_id != id)
    -- There are a lot of possible constraints here (e.g. `caption` and `text` can't appear on the same object).
    -- We will not implement those since all of our message objects come directly from the telegram API.
)
