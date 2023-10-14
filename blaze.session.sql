DROP TABLE warnings;

CREATE TABLE IF NOT EXISTS warnings (
    moderator_id BIGINT NOT NULL UNIQUE,
    user_id BIGINT NOT NULL UNIQUE,
    warning TEXT NOT NULL,
    warning_id UUID NOT NULL PRIMARY KEY
);

ALTER TABLE warnings
    ADD foreign key (moderator_id) references users(user_id) deferrable initially deferred,
    ADD foreign key (user_id) references users(user_id) deferrable initially deferred;