CREATE TABLE IF NOT EXISTS warnings (
    moderator_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    warning TEXT NOT NULL,
    warning_id BIGINT NOT NULL PRIMARY KEY
);