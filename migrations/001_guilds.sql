CREATE TABLE IF NOT EXISTS guilds (
    name                        TEXT NOT NULL,
    guild_id                    BIGINT NOT NULL,
    owner_id                    BIGINT NOT NULL,
    PRIMARY KEY (guild_id)
);