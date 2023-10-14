CREATE TABLE IF NOT EXISTS user_warnings (
    moderator_id BIGINT NOT NULL UNIQUE,
    user_id BIGINT NOT NULL UNIQUE,
    warning TEXT NOT NULL,
    warning_id BIGINT NOT NULL PRIMARY KEY
);