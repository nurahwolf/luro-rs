-- DROP TABLE guild_roles;
CREATE TABLE IF NOT EXISTS guild_roles (
    role_id bigint NOT NULL PRIMARY KEY,
    guild_id bigint NOT NULL references guilds(guild_id),
    
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
    tags JSONB,
    unicode_emoji TEXT
);