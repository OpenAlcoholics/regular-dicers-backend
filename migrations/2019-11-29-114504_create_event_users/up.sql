-- Your SQL goes here
CREATE TABLE IF NOT EXISTS event_users
(
    id        SERIAL UNIQUE PRIMARY KEY,
    event_id  Integer NOT NULL,
    user_id   Integer NOT NULL,
    attends   boolean NOT NULL DEFAULT false,
    UNIQUE (event_id, user_id),
    FOREIGN KEY (event_id) REFERENCES events (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
