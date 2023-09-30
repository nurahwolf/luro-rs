CREATE TYPE user_permissions AS ENUM (
    'USER', 'OWNER', 'ADMINISTRATOR'
);

CREATE TABLE IF NOT EXISTS users (
    name                VARCHAR(64) NOT NULL,
    user_id             INT8 NOT NULL,
    user_permissions    user_permissions NOT NULL,
    PRIMARY KEY (user_id)
);