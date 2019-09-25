-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_rolls
(
    id            SERIAL UNIQUE PRIMARY KEY,
    event_user_id Integer NOT NULL,
    callback_query_id   Text    NOT NULL,
    jumbo         boolean NOT NULL DEFAULT false,
    alcoholic     boolean NOT NULL DEFAULT false,
    roll          Integer NOT NULL,
    drink         Text    NOT NULL,
    UNIQUE (event_user_id, callback_query_id),
    FOREIGN KEY (event_user_id) REFERENCES event_users (id),
    constraint valid_roll
        check (((roll >= 0) AND (roll <= 6)) OR (roll is NULL))
);
