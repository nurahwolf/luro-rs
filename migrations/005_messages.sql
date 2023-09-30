CREATE TABLE IF NOT EXISTS messages (
    message_id                    INT8 NOT NULL PRIMARY KEY,
    author_id                     INT8 NOT NULL REFERENCES users(user_id),
    content                       text,
);