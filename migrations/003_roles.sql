CREATE TABLE IF NOT EXISTS roles (
    colour INT NOT NULL, 
    deleted BOOLEAN NOT NULL DEFAULT false,
    flags BIGINT NOT NULL DEFAULT 0,
    hoist BOOLEAN NOT NULL DEFAULT false,
    icon JSONB,
    managed BOOLEAN NOT NULL DEFAULT false,
    mentionable BOOLEAN NOT NULL DEFAULT false,
    name TEXT NOT NULL,
    permissions BIGINT NOT NULL DEFAULT 0,
    position BIGINT NOT NULL,
    role_id INT8 NOT NULL PRIMARY KEY,
    tags JSONB,
    unicode_emoji TEXT
);