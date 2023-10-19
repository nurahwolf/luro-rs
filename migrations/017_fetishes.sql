-- DROP TABLE fetishes;
CREATE TABLE IF NOT EXISTS fetishes (
    fetish_id       bigint NOT NULL primary key,

    name            text NOT NULL,
    description     text NOT NULL,
    creator         bigint references users (user_id) ON UPDATE CASCADE ON DELETE CASCADE
);