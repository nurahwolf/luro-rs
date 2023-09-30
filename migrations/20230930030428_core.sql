CREATE TABLE IF NOT EXISTS guilds (
    guild_id                    INT8 NOT NULL,
    PRIMARY KEY (guild_id)
);


CREATE TABLE IF NOT EXISTS roles (
    role_id     INT8 NOT NULL,
    PRIMARY KEY (role_id)

);

CREATE TYPE user_permissions AS ENUM (
    'USER', 'OWNER', 'ADMINISTRATOR'
);

CREATE TABLE IF NOT EXISTS users (
    name                VARCHAR(64) NOT NULL,
    user_id             INT8 NOT NULL,
    user_permissions    user_permissions NOT NULL,
    PRIMARY KEY (user_id)
);

CREATE TABLE IF NOT EXISTS interactions (
    application_id      INT8 NOT NULL,
    interaction_id      INT8 NOT NULL,
    message_id          INT8,
    kind                BYTEA NOT NULL,
    token               VARCHAR(64) NOT NULL,
    data                BYTEA NOT NULL,
    PRIMARY KEY (interaction_id)
);